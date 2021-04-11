use colored::Colorize;
use crossbeam_channel;
use std::sync::mpsc;

use crate::TraceError;
use crate::{
    ptrace::{SyscallPtrace, Tracer},
    TraceType,
};
use crate::{HTraceIterator, Syscall};
use crate::{TraceOptions, TraceOutput, TraceThread};

/// Actual main tracer
#[derive(Debug)]
pub struct HStrace {
    // What to trace
    trace_type: TraceType,

    // Extra tracing options
    tracing_options: TraceOptions,

    // Tracing event receiver
    syscall_event_receiver: Option<crossbeam_channel::Receiver<TraceOutput>>,

    // Quit channel receiver - tracing will stop when item is received
    exit_receiver: Option<crossbeam_channel::Receiver<()>>,
}

impl HStrace {
    pub fn new(
        trace_type: TraceType,
        tracing_options: TraceOptions,
        exit_receiver: Option<crossbeam_channel::Receiver<()>>,
    ) -> Self {
        Self {
            trace_type,
            tracing_options,
            exit_receiver,
            syscall_event_receiver: None,
        }
    }

    /// Starts the tracer
    pub fn start(&mut self) -> Result<(), TraceError> {
        let (startup_result_sender, startup_result_receiver) = mpsc::channel();

        let (trace_output_event_sender, trace_output_event_receiver) =
            crossbeam_channel::bounded(50);

        self.syscall_event_receiver = Some(trace_output_event_receiver);

        let options = self.tracing_options.clone();
        let set_exit_receiver = self.exit_receiver.clone();
        let trace_type = self.trace_type.clone();

        // here we launch a tracing thread, the thread will send an initial single message through startup_result_receiver
        std::thread::spawn(move || {
            run_tracing_thread(
                trace_type,
                &startup_result_sender,
                trace_output_event_sender,
                options,
                set_exit_receiver,
            );
        });

        // wait until the thread startup message arrives
        match startup_result_receiver
            .recv()
            .map_err(|_| TraceError::MpscError)?
        {
            Err(e) => Err(e),
            Ok(()) => Ok(()),
        }
    }

    pub fn iter(&mut self) -> crossbeam_channel::Iter<TraceOutput> {
        self.syscall_event_receiver.as_mut().unwrap().iter()
    }

    pub fn iter_grouped(&mut self) -> HTraceIterator {
        HTraceIterator::new(self.syscall_event_receiver.clone().unwrap())
    }

    /// FIXME hide the Map, othewise not good API to be published
    pub fn iter_as_syscall(
        &mut self,
    ) -> std::iter::Map<crossbeam_channel::Iter<TraceOutput>, impl FnMut(TraceOutput) -> Syscall>
    {
        self.syscall_event_receiver
            .as_mut()
            .unwrap()
            .iter()
            .map(|message| Syscall::from(message))
    }

    pub fn print_totals(&self) {
        println!(
            "{}",
            format!("-------------------------------------------------------------").blue()
        );
        println!(
            "{}:    0, {}: 2352kB",
            format!("Pids").magenta(),
            format!("Max memory usage").magenta()
        );

        println!(
            "Network: {}, {}",
            format!(
                "{} (main, {}/{})",
                "127.0.0.1:8080".cyan(),
                "52b".green(),
                "52b".green()
            ),
            format!(
                "{} (main, {}, {})",
                "10.5.2.1:8080".cyan(),
                "52b".green(),
                "52b".green()
            ),
        );

        println!(
            "Files:   {}, {}, (5 supressed)",
            format!("{} ({})", "/etc/passwd".cyan(), "RW".red(),),
            format!("{} ({})", "/usr/include/test.h".cyan(), "R".green(),),
        );

        println!(
            "{}:    run with --file-all to view all files",
            format!("Info").magenta()
        );

        println!(
            "^ above information is not real data, but displays a possibility of adding a summary"
        );
    }
}

// main tracing loop
fn run_tracing_thread(
    trace_type: TraceType,
    state_sender: &mpsc::Sender<Result<(), TraceError>>,
    sender: crossbeam_channel::Sender<TraceOutput>,
    options: TraceOptions,
    exit_receiver: Option<crossbeam_channel::Receiver<()>>,
) {
    // !!!! keep the ::new and initialize -calls close by, because we need to attach as soon as possible
    let mut ptrace = match SyscallPtrace::new(trace_type) {
        Ok(sd) => sd,
        Err(e) => {
            return state_sender.send(Err(e)).unwrap();
        }
    };

    // do initialize only when not in parent pid
    if let Err(e) = ptrace.initialize() {
        return state_sender.send(Err(e)).unwrap();
    }

    // launch the main tracer thread
    let mut tracer_thread = TraceThread::new(ptrace, sender, options);
    log::debug!("Tracer_thread started, going into loop");

    state_sender.send(Ok(())).unwrap();

    crossbeam_utils::thread::scope(|_s| {
        // message receiver loop
        loop {
            // quit channel
            if let Some(exit_receiver) = &exit_receiver {
                match exit_receiver.try_recv() {
                    Ok(_) => {
                        log::debug!("exit_receiver triggered, break loop");
                        break;
                    }

                    Err(e) => match e {
                        crossbeam_channel::TryRecvError::Empty => (),
                        crossbeam_channel::TryRecvError::Disconnected => {
                            log::debug!("exit_receiver disconnected, break loop");
                            break;
                        }
                    },
                }
            }

            // tracer
            match tracer_thread.iterate() {
                Err(e) => {
                    log::error!("Tracer thread sent an error: {:?}", e);
                    break;
                }
                Ok((has_more, child_pid)) => {
                    // detect fork
                    if let Some(pid) = child_pid {
                        // not used for now, but could maybe be expanded to run pid-strace in separate thread
                        log::debug!("Detected fork, new pid {}", pid);
                    }

                    if !has_more {
                        break;
                    }
                }
            }
        }
    })
    .unwrap();

    if let Err(e) = tracer_thread.finalize() {
        log::warn!("Finalizing tracer received error: {:?}", e);
    }
}
