//! Syscall tracing CLI & library made in Rust
//!
//! Hstrace makes it possible to trace the syscalls a specific program and its child programs/threads are doing.
//!
//! # Quick start
//!
//! ```
//! use hstrace::prelude::*;
//!
//! // initializes and starts tracing
//! let mut tracer = HStraceBuilder::new()
//!     .pid(1000).build();
//! tracer.start();
/*
//!     .unwrap();
//!
//! // iterates over each trace item
//! for call in tracer.iter_as_syscall() {
//!     match call.kind {
//!         SyscallKind::Openat(o) => {
//!             if o.flags.contains(call::OpenatMode::O_WRONLY) {
//!                 println!("File {} opened in write-mode ({:?})", o.pathname, o.flags);
//!             }
//!         }
//!         _ => (),
//!     }
//! }
*/

#![feature(test)]
#![feature(core_intrinsics)]
#![feature(arbitrary_enum_discriminant)]

#[macro_use]
extern crate num_derive;

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate lazy_static;

#[macro_use]
pub(crate) mod macros;

pub mod call;
pub mod prelude;
mod syscall;

mod enums;
mod from_c;

mod output;
pub mod ptrace;
mod trace;
mod trace_grouper;
mod traits;
pub mod value;
pub use output::*;

use crate::traits::hmz_format;
pub use call::SyscallKind;
pub(crate) use ptrace::Tracer;
pub use syscall::*;
pub use trace::*;

use serde::Serialize;

/// Resolved system call
#[derive(Serialize, Debug)]
pub struct Syscall {
    /// Call that was made (enum variant). Resolved to correct one in 95%+ of cases. If not known, contains `Ident::Unknown`
    pub name: Ident,

    /// Enum variant of syscall, contains call-specific data. Currently only a subset of syscalls are resolved to these expanded structures
    pub kind: SyscallKind,

    /// Result of the syscall (success, or an error)
    pub result: Result<(), SyscallError>,
}

impl Syscall {
    pub(crate) fn new(name: Ident, kind: SyscallKind, result: Result<(), SyscallError>) -> Self {
        Syscall { name, kind, result }
    }

    /// Return a string of syscall information in "human-readable" format
    pub fn fmt_human(&self) -> String {
        if let SyscallKind::None = self.kind {
            hmz_format(&format!("{:?}", self.name), "N/A")
        } else {
            self.kind.fmt_human()
        }
    }
}

pub(crate) trait MapFromC {
    type Item;
    fn from_c<T>(c: T) -> Self::Item;
}
