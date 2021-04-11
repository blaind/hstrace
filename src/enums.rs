/// ptrace(PTRACE_GET_SYSCALL_INFO) operation
#[derive(Debug, FromPrimitive)]
pub(crate) enum PtraceOp {
    None = 0,
    Entry = 1,
    Exit = 2,
    Seccomp = 3,
}
