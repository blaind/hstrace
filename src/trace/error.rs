use std::fmt;

/// An error returned when tracing fails
#[derive(PartialEq, Clone)]
pub enum TraceError {
    /// ptrace returned unknown op (should be either Entry or Exit)
    UnknownOp(usize),

    /// No output received
    NoOutput,

    /// Tracer found Exit data, but Entry data was omitted. Internal error
    NoMatchingEntryData,

    /// Syscall referenced memory location with too large size. This is to prevent misbehaving programs of running
    TooLargeMemoryReadRequested(usize, usize),

    /// Desired struct size didn't match the requested memory read size
    MemoryCapacityError(String, usize, usize),

    /// Detected duplicate Entry -data from ptrace. Should have had Exit in between - probably hstrace internal error
    DuplicateEntry,

    /// Process-related errors are wrapped here
    NixError(nix::Error),

    /// TracerThread exited, no more data
    MpscError,

    /// StringError: internal error (that is reported as string)
    StringError(String),

    /// TraceType was not set. Call the class with pid or program arguments
    NoTraceTypeSet,

    /// Tracee memory could not be read
    MemoryReadError(nix::Error, usize, usize, usize),

    /// Syscall ptrace(PTRACE_GET_SYSCALL_INFO, ...) failed. Must have Linux kernel 5.3. or newer
    PtraceGetSyscallInfo,

    /// to_type(..) conversion failed, this is hstrace internal error, please report
    ToTypeError,
}

impl fmt::Debug for TraceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TraceError::UnknownOp(op) => {
                write!(f, "UnknownOp: ptrace returned an unknown OP ({})", op)
            }

            TraceError::NoOutput => write!(f, "NoOutput: didn't receive ptrace output"),

            TraceError::NoMatchingEntryData => write!(f, "NoMatchingEntryData: ptrace returned syscall exit data, but did not catch entry data"),

            TraceError::TooLargeMemoryReadRequested(mem, max) => {
                write!(f, "TooLargeMemoryReadRequested: Trying to read too large section of a memory ({} > {}). This is a safety guard in hstrace to prevent DDoS by misbehaving code", mem, max)
            }

            TraceError::MemoryCapacityError(struct_name, requested_size, struct_size) => write!(f, "MemoryCapacityError: Tried to read {} bytes into struct {} which has size {}", requested_size, struct_name, struct_size),

            TraceError::DuplicateEntry => write!(f, "DuplicateEntry: Detected duplicate Entry -data from ptrace. Should have had Exit in between - probably hstrace internal error"),

            TraceError::NixError(e) => write!(f, "NixError: {:?}", e),

            TraceError::MpscError => write!(f, "MpscError: TracerThread exited, no more data"),

            TraceError::StringError(e) => write!(f, "StringError: internal error {:?}", e),

            TraceError::NoTraceTypeSet => write!(f, "NoTraceTypeSet: TraceType was not set. Call the class with pid or program arguments"),

            TraceError::MemoryReadError(e, address, len, pid) => write!(f, "MemoryReadError: Tracee memory could not be read (tried to read from pid {} at address {} for {} bytes, received error {:?}", pid, address, len, e),

            TraceError::PtraceGetSyscallInfo => write!(f, "PtraceGetSyscallInfo: syscall ptrace(PTRACE_GET_SYSCALL_INFO, ...) failed. Must have Linux kernel 5.3. or newer"),

            TraceError::ToTypeError => write!(f, "ToTypeError: to_type(..) conversion failed, this is hstrace internal error, please report"),
        }
    }
}
