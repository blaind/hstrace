use nix::libc;
use nix::sys::{ptrace, wait};
use nix::unistd::Pid;
use std::{mem, process::Child, process::Command};

use super::{GetPtraceInfo, Tracer};
use crate::from_c;
use crate::TraceError;
use crate::TraceType;

#[derive(Debug)]
pub struct SyscallPtrace {
    pid: usize,
    cmd: Option<Child>,
    iteration: usize,
    current_pid: usize,
    is_exited: bool,
    is_finalized: bool,
}

const PTRACE_EVENT_STOP: i32 = 128; // FIXME move to system-dependant

impl SyscallPtrace {
    pub fn new(trace_type: TraceType) -> Result<Self, TraceError> {
        let (pid, cmd) = match trace_type {
            TraceType::Program(command, args) => {
                let cmd = Command::new(&command)
                    .args(&args)
                    //.stdout(std::process::Stdio::null())
                    //.stderr(std::process::Stdio::null())
                    .spawn()
                    .map_err(|e| TraceError::StringError(format!("{:?}", e)))?;

                let pid = cmd.id() as usize;

                log::debug!(
                    "Spawned command {:?} with args {:?} -> PID: {:?}",
                    command,
                    args,
                    pid
                );

                (pid, Some(cmd))
            }
            TraceType::Pid(pid) => (pid, None),
        };

        let current_pid = pid.clone();

        Ok(SyscallPtrace {
            pid,
            cmd,
            iteration: 0,
            current_pid,
            is_exited: false,
            is_finalized: false,
        })
    }

    #[inline]
    fn run_syscall(&self) -> Result<(), TraceError> {
        log::debug!("ptrace(PTRACE_SYSCALL, {}, ...)", self.current_pid);

        if let Err(e) = ptrace::syscall(Pid::from_raw(self.current_pid as i32), None) {
            log::error!(
                "ptrace(PTRACE_SYSCALL, {}) error: {:?}",
                self.current_pid,
                e
            );

            return Err(TraceError::NixError(e));
        }

        Ok(())
    }

    /// Waitpid loop
    #[inline]
    fn run_waitpid(&mut self) -> Result<bool, TraceError> {
        let ws = match wait::waitpid(
            Pid::from_raw(-1),
            Some(
                wait::WaitPidFlag::__WALL
                    | wait::WaitPidFlag::WSTOPPED
                    | wait::WaitPidFlag::WCONTINUED,
            ),
        ) {
            Err(e) => {
                log::error!("wait::waitpid(-1) error: {:?}", e);
                return Err(TraceError::NixError(e));
            }
            Ok(ws) => ws,
        };

        match ws {
            wait::WaitStatus::PtraceEvent(pid, _, event) => {
                let pid = pid.as_raw() as usize;

                match event {
                    libc::PTRACE_EVENT_FORK
                    | libc::PTRACE_EVENT_VFORK
                    | libc::PTRACE_EVENT_CLONE
                    | libc::PTRACE_EVENT_EXEC
                    | libc::PTRACE_EVENT_EXIT
                    | PTRACE_EVENT_STOP => {
                        // fork event, continue ptrace
                        log::debug!("wait::waitpid(-1) -> (pid: {}, event: {})", pid, event);
                        self.current_pid = pid;
                        self.run_syscall()?;
                        self.run_waitpid()?;
                    }

                    _ => {
                        panic!("wait::waitpid(-1) -> Unknown ptrace_event: {:?}", event);
                    }
                }
            }

            wait::WaitStatus::PtraceSyscall(pid) => {
                let pid = pid.as_raw() as usize;

                // ptrace_wait, switch pid if different
                if self.current_pid != pid {
                    log::debug!(
                        "wait::waitpid(-1) = {} (previous pid was {})",
                        pid,
                        self.current_pid
                    );
                    self.current_pid = pid;
                } else {
                    log::debug!("wait::waitpid(-1) = {}", pid);
                }
            }

            wait::WaitStatus::Exited(pid, status) => {
                let pid = pid.as_raw() as usize;

                // pid exited, run waitpid again
                if pid != self.pid {
                    // if not root pid, continue waiting
                    log::debug!(
                        "wait::waitpid(-1) -> child process exited (pid: {}, status: {})",
                        pid,
                        status
                    );

                    self.run_waitpid()?;
                } else {
                    log::debug!(
                        "wait::waitpid(-1) -> main exited (pid: {}, status: {})",
                        pid,
                        status
                    );

                    self.is_exited = true;

                    return Ok(false);
                }
            }

            _ => {
                log::debug!("wait::waitpid(-1) -> Unknown event: {:?}", ws);
            }
        }

        Ok(true)
    }
}

impl Drop for SyscallPtrace {
    fn drop(&mut self) {
        if !self.is_finalized {
            // make sure that finalize happens
            self.finalize().unwrap();
        }
    }
}

impl Tracer for SyscallPtrace {
    #[inline]
    fn initialize(&mut self) -> Result<(), TraceError> {
        if let Err(e) = ptrace::seize(
            Pid::from_raw(self.pid as i32),
            ptrace::Options::PTRACE_O_TRACESYSGOOD
                | ptrace::Options::PTRACE_O_TRACEFORK
                | ptrace::Options::PTRACE_O_TRACEVFORK
                | ptrace::Options::PTRACE_O_TRACECLONE
                | ptrace::Options::PTRACE_O_TRACEEXEC
                | ptrace::Options::PTRACE_O_TRACEEXIT,
        ) {
            log::debug!("ptrace::seize ERROR: {:?}", e);
            return Err(TraceError::NixError(e));
        }

        //if let Err(e) = ptrace::interrupt(Pid::from_raw(self.pid as i32)) { // see https://github.com/nix-rust/nix/pull/1422
        if let Err(e) = unsafe {
            ptrace::ptrace(
                ptrace::Request::PTRACE_INTERRUPT,
                Pid::from_raw(self.pid as i32),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            )
        } {
            log::error!("ptrace_interrupt error in initialization: {:?}", e);
            return Err(TraceError::NixError(e));
        }

        match wait::waitpid(Pid::from_raw(self.pid as i32), None) {
            Err(e) => {
                log::error!("Waitpid error in initialization: {:?}", e);
                return Err(TraceError::NixError(e));
            }
            Ok(_ws) => {
                //println!("Received ws {:?}", ws);
                // TODO: should we check that the WaitStatus matches `PtraceEvent(Pid(self.pid), SIGTRAP, 128`?
            }
        }

        log::debug!("Initialize ok");

        Ok(())
    }

    #[inline]
    fn prepare_next(&mut self) -> Result<bool, TraceError> {
        self.run_syscall()?;
        let has_more = self.run_waitpid()?;

        Ok(has_more)
    }

    #[inline]
    fn get_ptrace(
        &mut self,
        data_ptr: *mut from_c::ptrace_syscall_info,
    ) -> Result<GetPtraceInfo, TraceError> {
        log::debug!(
            "libc::ptrace(PTRACE_GET_SYSCALL_INFO, {}, ...)",
            self.current_pid
        );

        let ret_bytes = unsafe {
            libc::ptrace(
                from_c::PTRACE_GET_SYSCALL_INFO,
                libc::pid_t::from(Pid::from_raw(self.current_pid as i32)),
                mem::size_of::<from_c::ptrace_syscall_info>(),
                data_ptr,
            )
        };

        if ret_bytes < 0 {
            log::debug!(
                "ptrace(PTRACE_GET_SYSCALL_INFO, ...) returned {}",
                ret_bytes
            );

            if self.iteration > 0 {
                // at least one success, probably no more data, return
                return Ok(GetPtraceInfo {
                    has_more: false,
                    pid: self.current_pid,
                });
            } else {
                // no successes, kernel might not support this operation
                return Err(TraceError::PtraceGetSyscallInfo);
            }
        }

        self.iteration += 1;

        Ok(GetPtraceInfo {
            has_more: true,
            pid: self.current_pid,
        })
    }

    fn finalize(&mut self) -> Result<(), TraceError> {
        self.is_finalized = true;

        if let Some(mut cmd) = self.cmd.take() {
            if !self.is_exited {
                match cmd.kill() {
                    Err(e) => {
                        log::debug!(
                            "Could not kill command, error: {:} (this is probably ok)",
                            e
                        );
                    }
                    Ok(_) => {
                        log::debug!("cmd killed");
                    }
                }
            }
        }

        Ok(())
    }

    #[inline]
    fn read_memory_to_destination<T>(
        &mut self,
        pid: usize,
        address: usize,
        dest: *mut T,
    ) -> Result<(), TraceError> {
        let len = mem::size_of::<T>();
        let slice = unsafe { std::slice::from_raw_parts_mut(dest as *mut u8, len) };

        let read_len = match nix::sys::uio::process_vm_readv(
            Pid::from_raw(pid as i32),
            &[nix::sys::uio::IoVec::from_mut_slice(slice)],
            &[nix::sys::uio::RemoteIoVec { base: address, len }],
        ) {
            Ok(read_len) => read_len,
            Err(e) => return Err(TraceError::MemoryReadError(e, address, len, pid)),
        };

        assert_eq!(
            read_len, len,
            "Read enough memory in read_memory_to_destination"
        );

        Ok(())
    }

    #[inline]
    fn find_string_from_memory(
        &mut self,
        pid: usize,
        address: usize,
    ) -> Result<String, TraceError> {
        // FIXME 1656 used by strace, where does it come from?
        let buf = self.read_memory_to_vec(pid, address, 16560)?; // FIXME how to determine len? use 1656
        let null_byte_pos = match buf.iter().position(|b| *b == 0) {
            Some(pos) => pos,
            None => {
                panic!("Could not find null byte position, buffer: {:?}", buf);
            }
        };

        let slice = &buf[..null_byte_pos + 1];
        let c_str = std::ffi::CStr::from_bytes_with_nul(slice).unwrap();

        assert!(null_byte_pos <= buf.len());

        // FIXME this fails if we try to read utf8 and it's not utf8
        let ret = match c_str.to_str() {
            Ok(st) => st.to_owned(),
            Err(e) => {
                return Err(TraceError::StringError(format!(
                    "C_str conv: {:?}; addr 0x{:x} nullbyte {}",
                    e, address, null_byte_pos
                )));
            }
        };

        Ok(ret)
    }

    #[inline]
    fn read_memory_to_vec(
        &mut self,
        pid: usize,
        address: usize,
        len: usize,
    ) -> Result<Vec<u8>, TraceError> {
        self.assert_memory_len(len)?;

        if address == 0 {
            return Err(TraceError::StringError("address = nullptr".to_string()));
        }

        // FIXME add limiter

        log::debug!(
            "Read memory (pid: {}), address: {}, len: {}",
            pid,
            address,
            len
        );

        let mut buf = vec![0u8; len];

        let ret = match nix::sys::uio::process_vm_readv(
            Pid::from_raw(pid as i32),
            &[nix::sys::uio::IoVec::from_mut_slice(&mut buf)],
            &[nix::sys::uio::RemoteIoVec { base: address, len }],
        ) {
            Ok(ret) => ret,
            Err(e) => {
                return Err(TraceError::StringError(format!(
                    "read_memory_to_vec error: {:?}, pid {}, address {} len {}",
                    e, pid, address, len
                )));
            }
        };

        if ret < buf.len() {
            buf.truncate(ret);
        }

        assert!(ret <= len);

        Ok(buf)
    }

    #[inline]
    fn get_pid(&self) -> usize {
        self.current_pid
    }
}
