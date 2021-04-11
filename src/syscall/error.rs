/// This is re-export of `nix::errno::Errno`
pub use nix::errno::Errno;

use serde::Serialize;

#[derive(Debug, PartialEq)]
pub struct SyscallError {
    errno: Errno,
}

impl Serialize for SyscallError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i32(self.errno as i32)
    }
}

impl SyscallError {
    pub fn from_i32(ret: i32) -> Self {
        Self {
            errno: Errno::from_i32(ret),
        }
    }

    pub fn from_errno(errno: Errno) -> Self {
        Self { errno }
    }

    pub fn to_errno(&self) -> Errno {
        self.errno
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_syscall_error() {
        let err: SyscallError = SyscallError::from_i32(32);
        assert_eq!(err.to_errno(), Errno::EPIPE);

        let err: SyscallError = SyscallError::from_i32(32323232);
        assert_eq!(err.to_errno(), Errno::UnknownErrno);
    }
}
