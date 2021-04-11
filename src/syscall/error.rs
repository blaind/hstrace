/// This is re-export of `nix::errno::Errno`
pub use nix::errno::Errno as SyscallError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_syscall_error() {
        let err: SyscallError = SyscallError::from_i32(32);
        assert_eq!(err, SyscallError::EPIPE);

        let err: SyscallError = SyscallError::from_i32(32323232);
        assert_eq!(err, SyscallError::UnknownErrno);
    }
}
