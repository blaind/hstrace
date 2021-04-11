use crate::value::Value;
use crate::value::ValueTransformer;
use colored::Colorize;

pub(crate) trait CToCall {
    fn from_src<'a, T>(src: &mut ValueTransformer<'a, T>) -> Result<Value, crate::TraceError>
    where
        T: crate::Tracer;
}

/// Trait for formatting the Syscall Kind to human-readable format
pub trait Humanize {
    fn hmz(&self) -> String;
}

pub fn hmz_format(sysname: &str, args: &str) -> String {
    format!(
        "{}{} {}",
        format!("{}", sysname).yellow(),
        ":".purple(),
        args
    )
}
