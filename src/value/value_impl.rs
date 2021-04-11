use colored::Colorize;
use std::fmt;

use crate::call;
use crate::from_c;
use crate::value::helpers;
use crate::value::kind::*;
use crate::TraceError;
use crate::{call::CStruct, VarOutType};

// FIXME merge C_* to C_struct that contains an enum?
#[allow(non_camel_case_types)]
pub enum Value {
    // Primitive types
    Int(isize),
    UnsignedLong(usize),
    SizeT(usize),
    SSizeT(isize),
    OffT(usize),

    // Specialized types
    CString(String),
    Buffer(Vec<u8>),
    MemoryAddress(MemoryAddress),

    // Container types
    CStruct(CStruct),

    // Misc
    Void,
    None,
    Skip,
    Failure,
    Error(TraceError),
    ValueNotImplemented,

    // Move away
    AddressFamily(call::AddressFamily),
    C_stat(from_c::stat),
    SockAddr(call::SockAddr),
    SocketType(call::SocketType),
    //SwapFlag(call::SwapFlag),
    OpenatMode(call::OpenatMode),
    AccessMode(call::AccessMode),
    Prot(call::Prot),

    DynType(Box<dyn VarOutType>),
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Primitive types
            Value::Int(v) => write!(f, "{}", v),
            Value::UnsignedLong(v) => write!(f, "{}", v),
            Value::SizeT(v) => write!(f, "{}", v),
            Value::SSizeT(v) => write!(f, "{}", v),
            Value::OffT(v) => write!(f, "{}", v),

            // Specialized types
            Value::CString(v) => write!(f, "{:?}", v),
            Value::Buffer(v) => write!(f, "{}", helpers::format_buf(v, v.len() as i64, 32)),
            Value::MemoryAddress(v) => write!(f, "{:?}", v),

            // Container types
            Value::CStruct(cs) => write!(f, "{:?}", cs),

            // Error types
            Value::Void => write!(f, "void"),
            Value::None => write!(f, "None"),
            Value::Skip => write!(f, "?"),
            Value::Failure => write!(f, "{}", format!("FAIL").red()),
            Value::Error(e) => write!(f, "{}: {:?}", format!("FAIL").red(), e),
            Value::ValueNotImplemented => {
                write!(f, "{}", format!("https://git.io/Jv49L").magenta().dimmed())
            }

            // Move to somewhere else
            Value::AddressFamily(v) => write!(f, "{:?}", v),
            Value::C_stat(v) => write!(f, "{:?}", v),
            Value::SockAddr(v) => write!(f, "{:?}", v),
            Value::SocketType(v) => write!(f, "{:?}", v),
            //Value::SwapFlag(v) => write!(f, "{:?}", v),
            Value::OpenatMode(v) => write!(f, "{:?}", v),
            Value::AccessMode(v) => write!(f, "{:?}", v),
            Value::Prot(v) => write!(f, "{:?}", v),
            Value::DynType(_) => write!(f, "dyn VarOutType"),
        }
    }
}
