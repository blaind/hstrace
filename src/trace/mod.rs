mod error;
pub use error::*;

mod hstrace_builder;
pub use hstrace_builder::*;

mod hstrace_impl;
pub use hstrace_impl::*;

mod hstrace_iterator;
pub use hstrace_iterator::*;

mod thread;
pub use thread::*;

mod output;
pub use output::*;

mod options;
pub use options::*;

mod syscall_parameters;
pub(crate) use syscall_parameters::*;

/// What to trace
#[derive(Clone, Debug)]
pub enum TraceType {
    Pid(usize),
    Program(String, Vec<String>),
}
