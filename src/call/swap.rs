use super::prelude::*;
use serde::Serialize;

pub(crate) fn get_definitions(inp: &mut Definitions) {
    inp.add(
        Ident::Swapon,
        vec!["path", "swapflags"],
        vec![AV::CString(In), AV::Int(In)], // AV::DynType(In, SwapFlagConverter::new())],
        AV::Int(Out),
    );

    inp.add(
        Ident::Swapoff,
        vec!["path"],
        vec![AV::CString(In)],
        AV::Int(Out),
    );
}

/// Syscall: Start swapping to file/device
#[derive(Debug, PartialEq, FromPtrace, Serialize)]
#[hstrace(hmz("Enable swap for path {:?} with flags {:?}", self.path, self.swapflags))]
pub struct Swapon {
    #[hstrace]
    pub path: String,

    #[hstrace]
    pub swapflags: isize, // DynType<SwapFlag>,
}

/// Syscall: Stop swap on file/device
#[derive(Debug, PartialEq, FromPtrace, Serialize)]
#[hstrace(hmz("Disable swap path {}", self.path))]
pub struct Swapoff {
    #[hstrace]
    pub path: String,
}

bitflags! {
    pub struct SwapFlag: isize {
        // FIXME make platform-dependent
        const SWAP_FLAG_PREFER = 0x8000;
        const SWAP_FLAG_PRIO_MASK = 0x7fff;
        const SWAP_FLAG_PRIO_SHIFT = 0;
        const SWAP_FLAG_DISCARD = 0x10000;
    }
}

/*
struct SwapFlagConverter {}
impl SwapFlagConverter {
    pub fn new() -> Box<Self> {
        Box::new(Self {})
    }
}
impl VarType for SwapFlagConverter {
    fn convert(&self, input: u64) -> Value {
        Value::DynType(Box::new(SwapFlag::from_bits_truncate(input as isize)))
    }
}

impl VarOutType for SwapFlag {}
 */

/*
impl PtraceConversion for Swapon {
    fn process_entry<'a, T: crate::Tracer>(mut ptrace_args: PtraceArgs<T>) -> Self {
        Swapon {
            path: ptrace_args.convert_field(0),
            swapflags: SwapFlag::from_bits(ptrace_args.convert_field(1)).unwrap(),
        }
    }

    fn process_exit<'a, T: crate::Tracer>(&mut self, ptrace_args: PtraceArgs<T>) {}
}
*/
