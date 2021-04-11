use std::convert::TryFrom;

use crate::{SyscallError, VarType};

mod transformer;

pub(crate) use transformer::*;

mod helpers;

pub mod kind;

mod value_impl;
pub use value_impl::*;

use crate::call::CStructAV;
pub use crate::ptrace::MockPtrace;
use crate::syscall::Direction;

#[allow(non_camel_case_types)]
pub(crate) enum AV {
    // Primitive types
    Int(Direction),
    #[allow(dead_code)]
    UnsignedLong(Direction),
    SizeT(Direction),
    SSizeT(Direction),
    OffT(Direction),

    // Specialized types
    CString(Direction),
    BufferFromArgPosition(Direction, usize),
    BufferFromReturn(Direction),
    MemoryAddress(Direction),

    // Container types
    CStruct(Direction, CStructAV),

    // Misc
    Void(Direction),

    // Move away
    AddressFamily(Direction),
    SocketType(Direction),
    Prot(Direction),
    OpenatMode(Direction),
    AccessMode(Direction),
    SockAddr(Direction, usize),

    #[allow(dead_code)]
    DynType(Direction, Box<dyn VarType>),
}

pub(crate) fn map_output(out: &AV, is_error: u8, rval: i64) -> Result<Value, SyscallError> {
    if is_error > 0 {
        match i32::try_from(
            rval.checked_mul(-1)
                .ok_or_else(|| SyscallError::UnknownErrno)?,
        ) {
            Ok(val) => Err(SyscallError::from_i32(val)),
            Err(_) => return Err(SyscallError::UnknownErrno),
        }
    } else {
        Ok(match out {
            AV::Int(_) => Value::Int(rval as isize),
            AV::MemoryAddress(_) => Value::MemoryAddress(kind::MemoryAddress(rval as usize)),
            AV::SSizeT(_) => Value::SSizeT(rval as isize),
            AV::OffT(_) => Value::OffT(rval as usize),
            AV::Void(_) => Value::Void,
            _ => Value::Failure,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_map_output_overflow() {
        assert_eq!(
            map_output(&AV::Int(Direction::Out), 23, -9223372036854775808).unwrap_err(),
            SyscallError::UnknownErrno
        );
    }

    #[test]
    pub fn test_map_output() {
        assert_eq!(
            map_output(&AV::Int(Direction::Out), 23, -1).unwrap_err(),
            SyscallError::EPERM
        );

        match map_output(&AV::Int(Direction::Out), 0, 50).unwrap() {
            Value::Int(val) => assert_eq!(val, 50),
            _ => panic!("map_output expected value::int"),
        }
    }
}
