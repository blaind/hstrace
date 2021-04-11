use serial_test::serial;
use std::env;

use hstrace::value::kind::MemoryAddress;
use hstrace::{call, Ident};
use hstrace::{prelude::*, Errno, HStraceBuilder, SyscallError};

fn init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

// "Swapoff" is a trigger in c code, after which the order of tests will start
fn compare<'a>(sc: &'a hstrace::Syscall) -> bool {
    sc.name != Ident::Swapoff
}

fn get_st(ops: &str) -> HStrace {
    let mut hstrace = HStraceBuilder::new()
        .program("data/c_code/main")
        .arg(ops)
        .build();

    hstrace.start().unwrap();
    hstrace
}

macro_rules! unwrap_syscall {
    ($iter:expr, $expected:tt) => {{
        let next = $iter.next().unwrap();
        assert_eq!(next.name, Ident::$expected);
        match next.kind {
            SyscallKind::$expected(inner) => inner,
            _ => panic!("SyscallKind did not match Ident"),
        }
    }};
}

macro_rules! test_syscall {
    ($iter:expr, $expected:tt, $expected_container:expr, $expected_result:expr) => {
        let next = $iter.next().unwrap();
        assert_eq!(next.name, Ident::$expected);
        assert_eq!(next.kind, SyscallKind::$expected($expected_container));
        assert_eq!(next.result, $expected_result);
    };

    ($iter:expr, $expected:tt, $expected_container:expr) => {
        let next = $iter.next().unwrap();
        assert_eq!(next.name, Ident::$expected);
        assert_eq!(next.kind, SyscallKind::$expected($expected_container));
    };

    ($iter:expr, $expected:tt) => {
        let next = $iter.next().unwrap();
        assert_eq!(next.name, Ident::$expected);
    };
}

#[test]
#[serial]
fn test_sched() {
    init();

    let mut st = get_st("sched");
    let mut iterator = st.iter_as_syscall().skip_while(compare).skip(1);

    // main
    test_syscall!(iterator, Mmap);
    test_syscall!(iterator, Clone);

    // child 1
    test_syscall!(
        iterator,
        Readlink,
        call::Readlink {
            src: "/tmp/link_src_child_1".into(),
            dst: None,
        }
    );

    test_syscall!(iterator, Mmap);
    test_syscall!(iterator, Clone);
    // test_syscall!(iterator, exit);

    // child 2
    test_syscall!(
        iterator,
        Readlink,
        call::Readlink {
            src: "/tmp/link_src_child_2".into(),
            dst: None,
        }
    );
    test_syscall!(iterator, Wait4);
    // test_syscall!(iterator, exit);

    // main
    test_syscall!(iterator, Munmap);
    test_syscall!(iterator, Wait4);

    test_syscall!(iterator, Munmap);
}

#[test]
#[serial]
fn test_unistd() {
    init();

    let mut st = get_st("unistd");
    let mut iterator = st.iter_as_syscall().skip_while(compare).skip(1);

    test_syscall!(
        iterator,
        Access,
        call::Access {
            pathname: "/tmp".into(),
            mode: call::AccessMode::R_OK | call::AccessMode::W_OK,
        }
    );

    test_syscall!(
        iterator,
        Access,
        call::Access {
            pathname: "/tmp".into(),
            mode: call::AccessMode::F_OK,
        }
    );

    test_syscall!(
        iterator,
        Getcwd,
        call::Getcwd {
            pathname: Some(
                env::current_dir()
                    .unwrap()
                    .into_os_string()
                    .into_string()
                    .unwrap(),
            ),
        }
    );

    test_syscall!(
        iterator,
        Readlink,
        call::Readlink {
            src: "/tmp/link_src".into(),
            dst: None,
        },
        Err(SyscallError::from_errno(Errno::ENOENT))
    );

    test_syscall!(iterator, ExitGroup);
}

#[test]
#[serial]
fn test_fncntl() {
    init();

    let mut st = get_st("fncntl");
    let mut iterator = st.iter_as_syscall().skip_while(compare).skip(1);

    test_syscall!(
        iterator,
        Openat,
        call::Openat {
            dirfd: 0,
            pathname: "/tmp/hstrace.test".into(),
            flags: call::OpenatMode::O_WRONLY | call::OpenatMode::O_APPEND,
        },
        Err(SyscallError::from_errno(Errno::ENOENT))
    );

    test_syscall!(iterator, ExitGroup);
}

#[test]
#[serial]
fn test_swap() {
    init();

    let mut st = get_st("swap");
    let mut iterator = st.iter_as_syscall().skip_while(compare).skip(1);

    test_syscall!(
        iterator,
        Swapon,
        call::Swapon {
            path: "/tmp/ptrace/swap".into(),
            swapflags: 65536, //call::SwapFlag::SWAP_FLAG_DISCARD,
        },
        Err(SyscallError::from_errno(Errno::EPERM))
    );

    test_syscall!(
        iterator,
        Swapoff,
        call::Swapoff {
            path: "/tmp/ptrace/swap".into(),
        },
        Err(SyscallError::from_errno(Errno::EPERM))
    );

    test_syscall!(iterator, ExitGroup);
}

#[test]
#[serial]
fn test_utsname() {
    init();

    let mut st = get_st("utsname");
    let mut iterator = st.iter_as_syscall().skip_while(compare).skip(1);

    let call = unwrap_syscall!(iterator, Uname);
    let uts_name = nix::sys::utsname::uname();
    assert_eq!(&call.utsname.sysname, uts_name.sysname());
    assert_eq!(&call.utsname.nodename, uts_name.nodename());
    assert_eq!(&call.utsname.release, uts_name.release());
    assert_eq!(&call.utsname.version, uts_name.version());
    assert_eq!(&call.utsname.machine, uts_name.machine());
}

#[test]
#[serial]
fn test_stat() {
    init();

    let mut st = get_st("stat");
    let mut iterator = st.iter_as_syscall().skip_while(compare).skip(1);

    let call = unwrap_syscall!(iterator, Stat);
    assert_eq!(call.pathname, "/____nonexistant");
    assert_eq!(call.stat.st_blocks, 0);
    assert_eq!(call.stat.st_size, 0);
}

#[test]
#[serial]
fn test_sendfile() {
    init();

    let mut st = get_st("sendfile");
    let mut iterator = st.iter_as_syscall().skip_while(compare).skip(1);

    test_syscall!(
        iterator,
        Sendfile,
        call::Sendfile {
            out_fd: 5,
            in_fd: 4,
            offset: MemoryAddress(0),
            count: 10,
        }
    );

    test_syscall!(
        iterator,
        Sendfile,
        call::Sendfile {
            out_fd: 6,
            in_fd: 3,
            offset: MemoryAddress(0),
            count: 10,
        }
    );

    test_syscall!(iterator, ExitGroup);
}

#[test]
#[serial]
fn test_socket() {
    init();

    let mut st = get_st("socket");
    let mut iterator = st.iter_as_syscall().skip_while(compare).skip(1);

    test_syscall!(
        iterator,
        Socket,
        call::Socket {
            domain: call::AddressFamily::AF_INET,
            socket_type: call::SocketType::SOCK_DGRAM,
            protocol: 0,
        }
    );

    test_syscall!(iterator, Setsockopt);
    test_syscall!(iterator, Sendto);
    test_syscall!(iterator, Connect);
    test_syscall!(iterator, Sendto);
    test_syscall!(iterator, Close, call::Close { fd: 3 });
    test_syscall!(
        iterator,
        Socket,
        call::Socket {
            domain: call::AddressFamily::AF_INET6,
            socket_type: call::SocketType::SOCK_DGRAM,
            protocol: 0,
        }
    );
    test_syscall!(iterator, Connect);

    let _next = iterator.next();
    // println!("Erred {:?}", next); // FIXME: output errors
    //assert_eq!(iter.next().unwrap().syscall, Ident::exit_group);
}
