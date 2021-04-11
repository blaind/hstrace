//! Implementation of syscalls
//!
//! If a syscall contains implementation in this module, the syscall input & output parameters are resolved
//! to a syscall-specific struct.
//!
//! This struct is contained in `SyscallKind` enum for each resolved `Syscall`
//!
//! Currently only a subset of syscalls are resolved to these expanded structures
//! Use this [GitHub issue 3](https://github.com/blaind/hstrace/issues/3) to request a new syscall implementation

use num_traits::FromPrimitive;
use serde::Serialize;
use std::fmt;

use crate::syscall::{Definition, Direction};
use crate::traits::{CToCall, Humanize};
use crate::value::{Value, ValueTransformer};
use crate::{Definitions, Ident, TraceOutput};
use crate::{Syscall, TraceError};

mod helpers;
mod prelude;

define_modules!(
    fncntl, mman, sendfile, socket, stat, swap, unistd, prctl, utsname, sysinfo, sched,
);

define_calls!(
    Readlink, Brk, Close, Mprotect, Access, Openat, Getcwd, Chdir, Socket, Swapon, Swapoff,
    Sendfile, Uname, Stat,
);

define_structs!(sys_utsname(Utsname), sys_stat(StatResult),);
