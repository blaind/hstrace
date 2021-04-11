use clap::App;
use colored::Colorize;
use env_logger;
use std::env;

use hstrace::{FilterMode, HStrace, TraceOptions, TraceType};

fn main() {
    init_logger();
    let mut options = parse_settings();
    let mut hstrace = initialize_hstrace(&mut options);

    if let Err(e) = hstrace.start() {
        println!("{}: {:?}", format!("Trace failed").red(), e);
        return;
    };

    display_output(&options, hstrace);
}

fn init_logger() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }

    env_logger::init();
}

fn register_quit_channel() -> crossbeam_channel::Receiver<()> {
    let (quit_sender, exit_receiver) = crossbeam_channel::bounded(1);

    ctrlc::set_handler(move || {
        println!("Received ctrl-c, quitting...");
        if let Err(e) = quit_sender.send(()) {
            log::debug!(
                "Could not send quit_sender msg, possibly receiving end died already: {:?}",
                e
            );
        }
    })
    .unwrap();

    exit_receiver
}

fn parse_settings() -> ParsedOptions {
    let yml = clap::load_yaml!("clap.yml");
    let m = App::from(yml)
        .setting(clap::AppSettings::TrailingVarArg)
        .get_matches();

    let trace_type = match m.value_of("prog") {
        Some(prog) => {
            let args: Vec<String> = m
                .values_of("prog")
                .unwrap_or(clap::Values::default())
                .skip(1)
                .map(|x| x.to_owned())
                .collect();

            TraceType::Program(prog.to_owned(), args)
        }
        None => {
            let pid = m.value_of("pid").unwrap().parse::<usize>().unwrap();
            TraceType::Pid(pid)
        }
    };

    let display_mode = match m.value_of("mode").unwrap() {
        "human" => DisplayMode::Human,
        "devnull" => DisplayMode::DevNull,
        "strace" => DisplayMode::Strace,
        "grouped" => DisplayMode::Grouped,
        _ => panic!("Unknown mode, try human or strace"),
    };

    let mut filter_calls = Vec::new();

    // FIXME move to loop
    let _expr: Vec<String> = m
        .values_of("expr")
        .unwrap_or(clap::Values::default())
        .map(|v| v.to_string())
        .map(|mut v| match v.find("=") {
            None => panic!("Expr must be in key=val format! (was {:?})", v),
            Some(pos) => {
                let value = v.split_off(pos);
                if v == "trace" {
                    filter_calls.push(value[1..].to_string());
                }
                v
            }
        })
        .collect();

    let filter = if filter_calls.len() > 0 {
        FilterMode::Calls(filter_calls)
    } else {
        FilterMode::None
    };

    let trace_options = TraceOptions {
        filter,
        strlen: m
            .value_of("strsize")
            .unwrap()
            .parse::<usize>()
            .map(|x| Some(x))
            .unwrap_or(None),
    };

    ParsedOptions {
        trace_type: Some(trace_type),
        display_mode,
        trace_options: Some(trace_options),
    }
}

enum DisplayMode {
    Human,
    DevNull,
    Strace,
    Grouped,
}

struct ParsedOptions {
    display_mode: DisplayMode,
    trace_type: Option<TraceType>,
    trace_options: Option<TraceOptions>,
}

fn initialize_hstrace(options: &mut ParsedOptions) -> HStrace {
    let trace_type = options.trace_type.take().unwrap();

    match &trace_type {
        TraceType::Program(prog, args) => {
            println!(
                "Tracing program {} with args {}",
                prog.cyan(),
                format!("{:?}", args).cyan()
            );
        }

        TraceType::Pid(pid) => {
            println!("Tracing PID {}", format!("{}", pid).cyan());
        }
    };

    let hstrace = HStrace::new(
        trace_type,
        options.trace_options.take().unwrap(),
        Some(register_quit_channel()),
    );

    hstrace
}

fn display_output(options: &ParsedOptions, mut hstrace: HStrace) {
    let max_msg_count = 4_000_000_000_000_000; // FIXME

    match &options.display_mode {
        DisplayMode::Human => {
            for msg in hstrace.iter_as_syscall().take(max_msg_count) {
                println!("{}", msg.fmt_human());
            }
        }

        DisplayMode::DevNull => {
            for msg in hstrace.iter().take(max_msg_count) {
                format!("{:?}", msg);
            }
        }

        DisplayMode::Strace => {
            for msg in hstrace.iter().take(max_msg_count) {
                println!("{:?}", msg);
            }
        }

        DisplayMode::Grouped => {
            for msg in hstrace.iter_grouped().take(max_msg_count) {
                if msg.calls.len() == 1 {
                    println!("{}", format!("{:?}", msg.calls[0]));
                } else {
                    println!("{}", "File operations for file something");
                    for call in msg.calls {
                        println!("    {:?}", call);
                    }
                }
            }
        }
    }

    hstrace.print_totals();
}
