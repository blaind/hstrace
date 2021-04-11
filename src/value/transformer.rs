use crate::call;
use crate::syscall::Direction;
use crate::value::helpers::{skip_if_2, skip_if_3};
use crate::value::kind::*;
use crate::value::Value;
use crate::value::AV;
use crate::TraceError;
use crate::Tracer;

use nix::libc;
use num_traits::FromPrimitive;
use std::mem;

pub(crate) struct ValueTransformer<'a, T>
where
    T: Tracer,
{
    pub pid: usize,
    pub ptrace: &'a mut T,
    pub var_type: &'a AV,
    pub previous_variable: Option<&'a Value>,
    pub variables: Option<&'a [Value]>,
    pub value: u64,
    pub dir: Direction,
    pub return_value: Option<i64>, // FIXME use result here
}

impl<'a, T> ValueTransformer<'a, T>
where
    T: Tracer,
{
    pub fn to_type<A>(&mut self) -> Result<A, crate::TraceError> {
        let mut dst: A = unsafe { std::mem::zeroed() };

        let address = match self.dir {
            Direction::Out => match self.previous_variable {
                Some(value) => match value {
                    Value::SizeT(addr) => *addr,
                    _ => return Err(TraceError::ToTypeError),
                },

                None => return Err(TraceError::ToTypeError),
            },
            Direction::In => self.value as usize,
            _ => return Err(TraceError::ToTypeError),
        };

        self.ptrace
            .read_memory_to_destination(self.pid, address, &mut dst as *mut A)?;

        Ok(dst)
    }

    /// FIXME refactor this to return Option<>, also run the value setting (e.g. getting from memory) only when direction matches
    /// performance and soundness improvement
    /// FIXME: where bit flags truncate is used, do actually checking and warn if extra flags are present
    ///
    pub fn to_value(&mut self) -> Result<Value, TraceError>
    where
        T: Tracer,
    {
        // if we must finally read OUT-value, read the memory address at IN-stage, and store it temporarily
        if self.dir == Direction::In {
            match &self.var_type {
                AV::CString(d) | AV::BufferFromReturn(d) | AV::CStruct(d, _) => {
                    if d == &Direction::Out {
                        return Ok(Value::SizeT(self.value as usize));
                    }
                }
                _ => (),
            }
        }

        let out = match &self.var_type {
            AV::CStruct(d, c) => c.map_value(d, self)?,

            AV::CString(d2) => {
                if *d2 == Direction::In && self.dir == Direction::In {
                    // input string, get directly
                    Value::CString(
                        self.ptrace
                            .find_string_from_memory(self.pid, self.value as usize)?,
                    )
                } else if *d2 == Direction::Out && self.dir == Direction::Out {
                    // output, get the memory address from previous temporary
                    match self.previous_variable.unwrap() {
                        Value::SizeT(address) => {
                            Value::CString(self.ptrace.find_string_from_memory(self.pid, *address)?)
                        }
                        _ => {
                            return Err(TraceError::StringError(
                                "prev_self.inp was None".to_string(),
                            ))
                        }
                    }
                } else {
                    Value::Skip
                }
            }

            AV::BufferFromArgPosition(d2, size_arg_position) => {
                skip_if_3(&self.dir.clone(), d2, || {
                    // get struct size (from next parameter)
                    let size = match &self.variables.as_ref().unwrap()[*size_arg_position] {
                        Value::SizeT(i) => *i,
                        _ => {
                            return Err(TraceError::StringError(format!(
                                "Failed to get size_t() from position {}",
                                *size_arg_position
                            )));
                        }
                    } as usize;

                    // FIXME limit read? to max print size...
                    Ok(Value::Buffer(self.ptrace.read_memory_to_vec(
                        self.pid,
                        self.value as usize,
                        size,
                    )?))
                })?
            }

            AV::BufferFromReturn(d2) => {
                if *d2 == Direction::Out && self.dir == Direction::Out {
                    // output, get the memory address from previous temporary
                    match self.previous_variable.unwrap() {
                        // FIXME use Result here
                        Value::SizeT(address) => match self.return_value {
                            Some(v) => {
                                if v > 0 {
                                    Value::Buffer(
                                        self.ptrace
                                            .read_memory_to_vec(self.pid, *address, v as usize)?,
                                    )
                                } else {
                                    Value::MemoryAddress(MemoryAddress(self.value as usize))
                                }
                            }

                            None => return Err(TraceError::StringError(
                                "Return_value was not present, should've been (this is internal error"
                                    .into(),
                            )),
                        },
                        _ => Value::Failure,
                    }
                } else {
                    Value::Failure
                }
            }

            AV::AddressFamily(d2) => skip_if_2(&self.dir, d2, || {
                Value::AddressFamily(
                    FromPrimitive::from_u64(self.value).unwrap_or(call::AddressFamily::Unknown), // FIXME ?
                )
            }),

            AV::SocketType(d2) => skip_if_2(&self.dir, d2, || {
                Value::SocketType(call::SocketType::from_bits_truncate(self.value as isize))
            }),

            AV::Prot(d2) => skip_if_2(&self.dir, d2, || {
                Value::Prot(call::Prot::from_bits_truncate(self.value as isize))
            }),

            /*
            AV::SwapFlag(d2) => skip_if_2(&self.dir, d2, || {
                Value::SwapFlag(call::SwapFlag::from_bits_truncate(self.value as isize))
            }),
             */
            AV::OpenatMode(d2) => skip_if_2(&self.dir, d2, || {
                Value::OpenatMode(call::OpenatMode::from_bits_truncate(self.value as isize))
            }),

            AV::AccessMode(d2) => skip_if_2(&self.dir, d2, || {
                Value::AccessMode(call::AccessMode::from_bits_truncate(self.value as isize))
            }),

            AV::MemoryAddress(d2) => skip_if_2(&self.dir, d2, || {
                Value::MemoryAddress(MemoryAddress(self.value as usize))
            }),

            AV::Int(d2) => skip_if_2(&self.dir, d2, || {
                /*println!(
                    "CONVERT INT {} to Value::Int: {:?}",
                    value,
                    value.to_ne_bytes()
                );*/

                Value::Int(self.value as isize)
            }),
            AV::UnsignedLong(d2) => {
                skip_if_2(&self.dir, d2, || Value::UnsignedLong(self.value as usize))
            }
            AV::SizeT(d2) => skip_if_2(&self.dir, d2, || Value::SizeT(self.value as usize)),
            AV::SSizeT(d2) => skip_if_2(&self.dir, d2, || Value::SSizeT(self.value as isize)), // FIXME flip bits)?
            AV::OffT(d2) => skip_if_2(&self.dir, d2, || Value::OffT(self.value as usize)),

            /*
            AV::C_sysinfo(d2) => {
                let mut sysinfo: from_c::sysinfo = unsafe { mem::zeroed() };
                helpers::read_memory_to_destination(
                    &pid,
                    value as usize,
                    &mut sysinfo as *mut from_c::sysinfo,
                );

                skip_if_2(&dir, d2, move || {
                    // FIXME: for some reason bindgen can't derive(Clone)
                    Value::C_sysinfo(from_c::sysinfo {
                        uptime: sysinfo.uptime,
                        loads: sysinfo.loads,
                        totalram: sysinfo.totalram,
                        freeram: sysinfo.freeram,
                        sharedram: sysinfo.sharedram,
                        bufferram: sysinfo.bufferram,
                        totalswap: sysinfo.totalswap,
                        freeswap: sysinfo.freeswap,
                        procs: sysinfo.procs,
                        pad: sysinfo.pad,
                        totalhigh: sysinfo.totalhigh,
                        freehigh: sysinfo.freehigh,
                        mem_unit: sysinfo.mem_unit,
                        _f: from_c::__IncompleteArrayFieldirection::new(),
                    })
                })
            }
            */
            AV::SockAddr(d, size_arg_position) => skip_if_3(&self.dir.clone(), d, || {
                // get struct size (from next parameter)
                let size = match &self.variables.as_ref().unwrap()[*size_arg_position] {
                    Value::Int(i) => *i as usize,
                    _ => {
                        return Err(TraceError::StringError(
                            "Could not find Int-value of desired field; internal error".to_string(),
                        ))
                    }
                };

                // read memory as sockaddr
                let mut sockaddr: libc::sockaddr = unsafe { mem::zeroed() };

                const MAX_SOCKADDR_SIZE: usize = 1024;
                let struct_size = mem::size_of::<libc::sockaddr>();
                if size > MAX_SOCKADDR_SIZE || size < struct_size {
                    return Err(TraceError::MemoryCapacityError(
                        "sockaddr".into(),
                        size,
                        struct_size,
                    ));
                }

                self.ptrace.read_memory_to_destination(
                    self.pid,
                    self.value as usize,
                    &mut sockaddr as *mut libc::sockaddr,
                )?;

                // interpret address_family from sockaddr
                let address_family: call::AddressFamily =
                    match FromPrimitive::from_u16(sockaddr.sa_family) {
                        Some(d) => d,
                        None => {
                            return Ok(Value::SockAddr(call::SockAddr::__unknown(
                                sockaddr.sa_family as usize,
                            )))
                        }
                    };

                // based on address_family, reinterpret the memory
                let ret = match address_family {
                    call::AddressFamily::AF_UNIX => {
                        let mut sockaddr_un: libc::sockaddr_un = unsafe { mem::zeroed() };
                        let struct_size = mem::size_of::<libc::sockaddr_un>();
                        if size > struct_size {
                            return Err(TraceError::MemoryCapacityError(
                                "sockaddr_un".to_string(),
                                size,
                                struct_size,
                            ));
                        }

                        self.ptrace.read_memory_to_destination(
                            self.pid,
                            self.value as usize,
                            &mut sockaddr_un as *mut libc::sockaddr_un,
                        )?;

                        Value::SockAddr(call::SockAddr::UNIX(sockaddr_un))
                    }

                    call::AddressFamily::AF_INET => {
                        let mut sockaddr_in: libc::sockaddr_in = unsafe { mem::zeroed() };
                        let struct_size = mem::size_of::<libc::sockaddr_in>();
                        if size > struct_size {
                            return Err(TraceError::MemoryCapacityError(
                                "sockaddr_in".to_string(),
                                size,
                                struct_size,
                            ));
                        }

                        self.ptrace.read_memory_to_destination(
                            self.pid,
                            self.value as usize,
                            &mut sockaddr_in as *mut libc::sockaddr_in,
                        )?;

                        Value::SockAddr(call::SockAddr::INET(sockaddr_in))
                    }

                    call::AddressFamily::AF_INET6 => {
                        let mut sockaddr_in6: libc::sockaddr_in6 = unsafe { mem::zeroed() };
                        let struct_size = mem::size_of::<libc::sockaddr_in6>();
                        if size > struct_size {
                            return Err(TraceError::MemoryCapacityError(
                                "sockaddr_in6".to_string(),
                                size,
                                struct_size,
                            ));
                        }

                        self.ptrace.read_memory_to_destination(
                            self.pid,
                            self.value as usize,
                            &mut sockaddr_in6 as *mut libc::sockaddr_in6,
                        )?;

                        Value::SockAddr(call::SockAddr::INET6(sockaddr_in6))
                    }

                    _ => Value::SockAddr(call::SockAddr::__unknown(address_family as usize)),
                };

                Ok(ret)
            })?,

            AV::Void(_) => Value::Void,

            AV::DynType(d2, converter) => {
                skip_if_2(&self.dir, d2, || converter.convert(self.value))
            }
        };

        Ok(out)
    }
}
