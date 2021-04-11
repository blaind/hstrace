use super::prelude::*;

pub(crate) fn get_definitions(inp: &mut Definitions) {
    inp.add(
        Ident::Readlink,
        vec!["pathname", "buf", "bufsiz"],
        vec![AV::CString(In), AV::BufferFromReturn(Out), AV::SizeT(In)], // FIXME get buf size from exit
        AV::SSizeT(Out),
    );

    inp.add(
        Ident::Access,
        vec!["pathname", "mode"],
        vec![AV::CString(In), AV::AccessMode(In)],
        AV::Int(Out),
    );

    inp.add(
        Ident::Getcwd,
        vec!["buf", "size"],
        vec![AV::CString(Out), AV::Int(In)],
        AV::Int(Out),
    );

    inp.add(Ident::Close, vec!["fd"], vec![AV::Int(In)], AV::SSizeT(Out));

    inp.add(
        Ident::Brk,
        vec!["addr"],
        vec![AV::MemoryAddress(In)],
        AV::MemoryAddress(Out),
    );

    inp.add(
        Ident::Chdir,
        vec!["path"],
        vec![AV::CString(Out)],
        AV::Int(Out),
    );

    inp.add(
        Ident::Read,
        vec!["fd", "buf", "count"],
        vec![AV::Int(In), AV::BufferFromReturn(Out), AV::SizeT(In)],
        AV::SSizeT(Out),
    );

    inp.add(
        Ident::Write,
        vec!["fd", "buf", "count"],
        vec![AV::Int(In), AV::BufferFromArgPosition(In, 2), AV::SizeT(In)],
        AV::SSizeT(Out),
    );

    inp.add(
        Ident::Fork,
        vec![],
        vec![],       // FIXME TODO!
        AV::Int(Out), // FIXME <-- should be pid_t
    );

    inp.add(
        Ident::ExitGroup,
        vec!["status"],
        vec![AV::Int(In)],
        AV::Void(Out),
    );

    inp.add(
        Ident::Lseek,
        vec!["fd", "offset", "whence"],
        vec![AV::Int(In), AV::OffT(In), AV::Int(In)],
        AV::OffT(Out),
    );
}

/// Syscall: Read value of a symbolic link
#[derive(Debug, PartialEq, FromPtrace)]
#[hstrace(hmz("{:?} symlink points to {:?}", self.src, self.dst))]
pub struct Readlink {
    /// Source file
    #[hstrace]
    pub src: String,

    /// Destination file
    #[hstrace]
    pub dst: Option<String>,
}

#[derive(Debug, PartialEq, FromPtrace)]
#[hstrace(hmz("Check path {:?} permissions for mode {:?}", self.pathname, self.mode))]
pub struct Access {
    #[hstrace]
    pub pathname: String,

    #[hstrace]
    pub mode: AccessMode,
}

#[derive(Debug, PartialEq, FromPtrace)]
#[hstrace(hmz("Resolved current path to {:?}", self.pathname))]
pub struct Getcwd {
    #[hstrace]
    pub pathname: Option<String>,
}

#[derive(Debug, PartialEq, FromPtrace)]
#[hstrace(hmz("Closed file descriptor {:?}", self.fd))]
pub struct Close {
    /// File descriptor
    #[hstrace]
    pub fd: isize,
}

#[derive(Debug, PartialEq, FromPtrace)]
#[hstrace(hmz("Request memory address expansion to {:?}", self.addr))]
pub struct Brk {
    /// Expand to memory address
    #[hstrace]
    pub addr: MemoryAddress,
}

#[derive(Debug, PartialEq, FromPtrace)]
#[hstrace(hmz("Change working directory to {:?}", self.path))]
pub struct Chdir {
    /// Working directory
    #[hstrace]
    pub path: String,
}

bitflags! {
    pub struct AccessMode: isize {
        const R_OK = 4;
        const W_OK = 2;
        const X_OK = 1;
        const F_OK = 0;
    }
}
