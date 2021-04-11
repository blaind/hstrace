use num_traits::FromPrimitive;

use crate::call::SYSCALLS;
use crate::syscall::Definition;
use crate::syscall::Direction;
use crate::value::ValueTransformer;
use crate::value::{Value, AV};
use crate::{from_c, Ident, Tracer};

pub(crate) struct SyscallParameters<'a> {
    pub(crate) pid: usize,
    pub(crate) nr: usize,
    pub(crate) args: &'a [u64; 6],
    pub(crate) exit: &'a from_c::ptrace_syscall_info__bindgen_ty_1__bindgen_ty_2,
    pub(crate) direction: Direction,
}

impl<'a> SyscallParameters<'a> {
    pub(crate) fn from(
        direction: Direction,
        pid: usize,
        data: &'a from_c::ptrace_syscall_info,
    ) -> Self {
        let entry = unsafe { &data.__bindgen_anon_1.entry };
        let nr = entry.nr as usize;
        let info = SyscallParameters {
            pid,
            nr,
            args: &entry.args,
            exit: unsafe { &data.__bindgen_anon_1.exit },
            direction,
        };

        info
    }

    pub(crate) fn get_definition(&self) -> Option<&'a Definition> {
        // FIXME cache this! -> make SYSCALLS to be a hashmap
        SYSCALLS.iter().find(|sc| sc.call_nr == self.nr)
    }

    pub(crate) fn get_ident(&self) -> Option<Ident> {
        FromPrimitive::from_usize(self.nr)
    }

    pub(crate) fn transform_value<T>(
        &self,
        idx: usize,
        ptrace: &mut T,
        var_type: &AV,
        variables: &Vec<Value>,
    ) -> Value
    where
        T: Tracer,
    {
        ValueTransformer {
            pid: self.pid,
            ptrace,
            var_type,
            previous_variable: match self.direction {
                Direction::In => None,
                Direction::Out => Some(&variables[idx]),
                Direction::InOut => {
                    panic!("This should not happen, wrong direction in value transformer")
                }
            },
            variables: Some(&variables),
            value: self.args[idx],
            dir: self.direction,
            return_value: match self.direction {
                Direction::In => None,
                Direction::Out => Some(self.exit.rval),
                Direction::InOut => {
                    panic!("This should not happen, wrong direction in value transformer")
                }
            },
        }
        .to_value()
        // store errors into value as a special value
        .unwrap_or_else(|e| Value::Error(e))
    }
}
