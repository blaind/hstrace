pub(crate) use super::{CStruct, CStructAV};
pub(crate) use crate::syscall::Direction::{In, InOut, Out};
// pub(crate) use crate::syscall::{VarOutType, VarType};
pub(crate) use crate::traits::CToCall;
pub(crate) use crate::traits::{hmz_format, Humanize};
pub(crate) use crate::value::kind::*;
pub(crate) use crate::value::Value;
pub(crate) use crate::value::ValueTransformer;
pub(crate) use crate::value::AV;
pub(crate) use crate::Definitions;
pub(crate) use crate::Ident;
pub(crate) use crate::TraceOutput;

pub(crate) use hstrace_derive::FromCStruct;
pub(crate) use hstrace_derive::FromPtrace;
pub(crate) use nix::libc;
pub(crate) use std::os::raw::c_char;

pub(crate) fn c_char_to_string(x: *const c_char) -> String {
    unsafe { std::ffi::CStr::from_ptr(x).to_string_lossy().into_owned() }
}
