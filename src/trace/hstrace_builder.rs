use crate::{HStrace, TraceOptions, TraceType};

/// Build hstrace
pub struct HStraceBuilder {
    // What to trace
    trace_type: Option<TraceType>,

    // Extra tracing options
    tracing_options: TraceOptions,

    // Quit channel receiver - tracing will stop when item is received
    exit_receiver: Option<crossbeam_channel::Receiver<()>>,
}

impl HStraceBuilder {
    pub fn new() -> Self {
        Self {
            trace_type: None,
            tracing_options: TraceOptions::default(),
            exit_receiver: None,
        }
    }

    /// Set the program to be started and traced. Do not use `pid` arg if `program` is used
    pub fn program(mut self, program: &str) -> Self {
        self.trace_type = Some(TraceType::Program(program.to_string(), Vec::new()));
        self
    }

    /// Set *single* argument for a program. If multiple arguments, use `args`
    pub fn arg(self, arg: &str) -> Self {
        self.args(vec![arg.to_owned()])
    }

    /// Sets arguments for the program
    pub fn args(mut self, args: Vec<String>) -> Self {
        match &self.trace_type {
            Some(tt) => match tt {
                TraceType::Program(prog, _) => {
                    self.trace_type = Some(TraceType::Program(prog.clone(), args.clone()));
                }
                _ => (),
            },

            None => (),
        }

        return self;
    }

    /// Set a pid to be traced. Tracer will attach to the pid when started. Do not use `program` arg if `pid` is used
    pub fn pid(mut self, pid: usize) -> Self {
        self.trace_type = Some(TraceType::Pid(pid));
        self
    }

    /// Set the options to be used when tracing
    pub fn options(mut self, options: TraceOptions) -> Self {
        self.tracing_options = options;
        self
    }

    /// Sets the `mpsc::Receiver<T>` to be used if tracing needs to be stopped while in progress
    pub fn set_exit_receiver(mut self, set_exit_receiver: crossbeam_channel::Receiver<()>) -> Self {
        self.exit_receiver = Some(set_exit_receiver);
        self
    }

    pub fn build(self) -> HStrace {
        HStrace::new(
            self.trace_type.unwrap(),
            self.tracing_options,
            self.exit_receiver,
        )
    }
}
