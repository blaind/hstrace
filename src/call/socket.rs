use super::prelude::*;
use serde::Serialize;

pub(crate) fn get_definitions(inp: &mut Definitions) {
    inp.add(
        Ident::Socket,
        vec!["domain", "type", "protocol"],
        vec![AV::AddressFamily(In), AV::SocketType(In), AV::Int(In)],
        AV::Int(Out),
    );

    inp.add(
        Ident::Connect,
        vec!["sockfd", "addr", "addrlen"],
        vec![AV::Int(In), AV::SockAddr(In, 2), AV::Int(In)], // FIXME use struct sockaddr, socklen_t addrlen
        AV::Int(Out),
    );
}

/// Syscall: Create an endpoint for communication
#[derive(Debug, PartialEq, FromPtrace, Serialize)]
#[hstrace(hmz("Open {:?} domain socket with type {:?} and protocol {:?}", self.domain, self.socket_type, self.protocol))]
pub struct Socket {
    #[hstrace]
    pub domain: AddressFamily,

    #[hstrace]
    pub socket_type: SocketType,

    #[hstrace]
    pub protocol: isize,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq)]
pub enum SockAddr {
    UNIX(libc::sockaddr_un),
    INET(libc::sockaddr_in),
    INET6(libc::sockaddr_in6),
    __unknown(usize),
}

/// Argument: Communication domain / protocol family used
#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Clone, FromPrimitive, PartialEq, Serialize)]
#[repr(isize)]
pub enum AddressFamily {
    AF_UNSPEC = libc::AF_UNSPEC as isize,
    AF_UNIX = libc::AF_UNIX as isize,
    AF_INET = libc::AF_INET as isize,
    AF_INET6 = libc::AF_INET6 as isize,
    Unknown = -1,
}

bitflags! {
    /// Argument: Socket communication semantics
    #[derive(Serialize)]
    pub struct SocketType: isize {
        const SOCK_STREAM = libc::SOCK_STREAM as isize;
        const SOCK_DGRAM = libc::SOCK_DGRAM as isize;
        const SOCK_SEQPACKET = libc::SOCK_SEQPACKET as isize;
        const SOCK_RAW = libc::SOCK_RAW as isize;
        const SOCK_RDM = libc::SOCK_RDM as isize;
        const SOCK_PACKET = libc::SOCK_PACKET as isize;

        const SOCK_CLOEXEC = libc::SOCK_CLOEXEC as isize;
        const SOCK_NONBLOCK = libc::SOCK_NONBLOCK as isize;
    }
}

#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Clone, FromPrimitive, PartialEq)]
#[repr(isize)]
pub enum SocketLevel {
    SOL_SOCKET = libc::SOL_SOCKET as isize,
    Unknown = -1,
}
