use super::prelude::*;
use crate::from_c::utsname as sys_utsname;

pub(crate) fn get_definitions(inp: &mut Definitions) {
    inp.add(
        Ident::Uname,
        vec!["utsname"],
        vec![AV::CStruct(Out, CStructAV::sys_utsname)],
        AV::Int(Out),
    );
}

#[derive(Debug, PartialEq, FromPtrace)]
#[hstrace(hmz("Detected uname to be {:?}", self.utsname))]
pub struct Uname {
    #[hstrace(c_struct = sys_utsname)]
    pub utsname: Utsname,
}

#[derive(Debug, Clone, PartialEq, FromCStruct)]
#[hstrace(c_struct = sys_utsname)]
pub struct Utsname {
    #[hstrace(c_char)]
    pub sysname: String,

    #[hstrace(c_char)]
    pub nodename: String,

    #[hstrace(c_char)]
    pub release: String,

    #[hstrace(c_char)]
    pub version: String,

    #[hstrace(c_char)]
    pub machine: String,
}

/*
impl PtraceConversion for Uname {
    fn process_entry<'a, T: crate::Ptrace>(mut ptrace_args: PtraceArgs<T>) -> Self {
        Uname { utsname: None }
    }

    fn process_exit<'a, T: crate::Ptrace>(&mut self, mut ptrace_args: PtraceArgs<T>) {
        self.utsname = Some(
            ptrace_args
                .convert_c_struct(0, crate::Direction::Out)
                .unwrap(),
        );
    }
}
 */
