use super::prelude::*;
use serde::Serialize;

pub(crate) fn get_definitions(inp: &mut Definitions) {
    inp.add(
        Ident::Openat,
        vec!["dirfd", "pathname", "flags"],
        vec![AV::Int(In), AV::CString(In), AV::OpenatMode(In)],
        AV::Int(Out),
    );

    inp.add(
        Ident::Fcntl,
        vec!["fd", "cmd"],
        vec![AV::Int(In), AV::Int(In)],
        AV::Int(Out),
    );
}

#[derive(Debug, PartialEq, FromPtrace, Serialize)]
#[hstrace(hmz("Open a file (dirfd: {}) {} with flags {:?}", self.dirfd, self.pathname, self.flags))]
pub struct Openat {
    #[hstrace]
    pub dirfd: isize,

    #[hstrace]
    pub pathname: String,

    #[hstrace]
    pub flags: OpenatMode,
    //pub mode_t: isize,
}

bitflags! {
    #[derive(Serialize)]
    pub struct OpenatMode: isize {
        const O_ACCMODE = 0o0003;
        const O_RDONLY = 0o0;
        const O_WRONLY = 0o01;
        const O_RDWR = 0o02;
        const O_CREAT = 0o0100;
        const O_EXCL = 0o0200;
        const O_NOCTTY = 0o0400;
        const O_TRUNC = 0o01000;
        const O_APPEND = 0o02000;
        const O_NONBLOCK = 0o04000;

        const O_CLOEXEC = 0o02000000;
    }
}
