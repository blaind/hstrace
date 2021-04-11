#![allow(unused)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_must_use)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::fmt;

impl fmt::Debug for stat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{st_mode=FIXME, st_rderv=FIXME...}}")
    }
}
