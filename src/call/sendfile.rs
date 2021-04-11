use super::prelude::*;

pub(crate) fn get_definitions(inp: &mut Definitions) {
    inp.add(
        Ident::Sendfile,
        vec!["out_fd", "in_fd", "offset", "count"],
        vec![
            AV::Int(In),
            AV::Int(In),
            AV::MemoryAddress(In),
            AV::SizeT(In),
        ],
        AV::SSizeT(Out),
    );
}

/// Syscall: Transfer data between file descriptors
#[derive(Debug, PartialEq, FromPtrace)]
#[hstrace(hmz("Transfer data to fd {:?} from fd {:?} offset {:?} len {:?}", self.out_fd, self.in_fd, self.offset, self.count))]
pub struct Sendfile {
    /// FD where data is sent
    #[hstrace]
    pub out_fd: isize,

    /// FD to read from
    #[hstrace]
    pub in_fd: isize,

    #[hstrace]
    pub offset: MemoryAddress,

    /// Number of bytes to copy
    #[hstrace]
    pub count: usize,
}
