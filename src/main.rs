use clap::App;
use colored::Colorize;
use env_logger;
use std::{env, path::PathBuf};

use hstrace::{FilterMode, HStrace, Output, TraceOptions, TraceType};

fn main() {
    init_logger();
    let mut options = parse_settings();
    log::debug!("Parsed options: {:#?}", options);

    let is_json = match options.display_mode {
        DisplayMode::Json => true,
        _ => false,
    };

    let mut out = Output::new(&options.output_file, is_json);
    let mut hstrace = initialize_hstrace(&mut options, &mut out);

    if let Err(e) = hstrace.start() {
        out.write(format!("{}: {:?}", format!("Trace failed").red(), e));
        return;
    };

    display_output(&options, hstrace, &mut out);
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
        eprintln!("Received ctrl-c, quitting...");
        if let Err(e) = quit_sender.send(()) {
            log::debug!(
                "Could not send quit_sender msg, possibly receiving end died already: {:?}",
                e
            );
        }

        // FIXME: should reset terminal here, e.g. after -o test.txt top ctrl-c will fail
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

    let mut display_mode = match m.value_of("mode").unwrap() {
        "human" => DisplayMode::Human,
        "devnull" => DisplayMode::DevNull,
        "strace" => DisplayMode::Strace,
        "grouped" => DisplayMode::Grouped,
        _ => panic!("Unknown mode, try human or strace"),
    };

    let mut filter_calls = Vec::new();

    let output_file = m.value_of("output_file").map(|f| PathBuf::from(f));
    if let Some(path) = &output_file {
        if path.extension() == Some(std::ffi::OsStr::new("json")) {
            log::debug!("Setting DisplayMode to Json, because file suffix is .json");
            display_mode = DisplayMode::Json;
        }
    }

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
        output_file,
    }
}

#[derive(Debug)]
enum DisplayMode {
    Json,
    Human,
    DevNull,
    Strace,
    Grouped,
}

#[derive(Debug)]
struct ParsedOptions {
    display_mode: DisplayMode,
    trace_type: Option<TraceType>,
    trace_options: Option<TraceOptions>,
    output_file: Option<PathBuf>,
}

fn initialize_hstrace(options: &mut ParsedOptions, out: &mut Output) -> HStrace {
    let trace_type = options.trace_type.take().unwrap();

    match &trace_type {
        TraceType::Program(prog, args) => {
            out.write(format!(
                "Tracing program {} with args {}",
                prog.cyan(),
                format!("{:?}", args).cyan(),
            ));
        }

        TraceType::Pid(pid) => {
            out.write(format!("Tracing PID {}", format!("{}", pid).cyan()));
        }
    };

    let hstrace = HStrace::new(
        trace_type,
        options.trace_options.take().unwrap(),
        Some(register_quit_channel()),
    );

    hstrace
}

fn display_output(options: &ParsedOptions, mut hstrace: HStrace, out: &mut Output) {
    let max_msg_count = 4_000_000_000_000_000; // FIXME

    match &options.display_mode {
        DisplayMode::Human => {
            for msg in hstrace.iter_as_syscall().take(max_msg_count) {
                out.write(format!("{}", msg.fmt_human()));
            }
        }

        DisplayMode::Json => {
            for msg in hstrace.iter_as_syscall().take(max_msg_count) {
                out.write_json(serde_json::to_string(&msg).unwrap());
            }
        }

        DisplayMode::DevNull => {
            for msg in hstrace.iter().take(max_msg_count) {
                format!("{:?}", msg);
            }
        }

        DisplayMode::Strace => {
            for msg in hstrace.iter().take(max_msg_count) {
                out.write(format!("{:?}", msg));
            }
        }

        DisplayMode::Grouped => {
            for msg in hstrace.iter_grouped().take(max_msg_count) {
                if msg.calls.len() == 1 {
                    out.write(format!("{}", format!("{:?}", msg.calls[0])));
                } else {
                    out.write(format!("{}", "File operations for file something"));
                    for call in msg.calls {
                        out.write(format!("    {:?}", call));
                    }
                }
            }
        }
    }

    hstrace.print_totals(out);
}
