use super::prelude::*;
use serde::Serialize;

pub(crate) fn get_definitions(inp: &mut Definitions) {
    inp.add(
        Ident::Mmap,
        vec!["addr", "length", "prot", "flags", "fd", "offset"],
        vec![
            AV::MemoryAddress(In),
            AV::SizeT(In),
            AV::Int(In),
            AV::Int(In),
            AV::Int(In),
            AV::OffT(In),
        ],
        AV::MemoryAddress(Out), // FIXME handle errors...
    );

    inp.add(
        Ident::Mprotect,
        vec!["addr", "len", "prot"],
        vec![AV::MemoryAddress(In), AV::SizeT(In), AV::Prot(In)],
        AV::Int(Out),
    );

    /*
    new.push(NewDefinition::new(
        Ident::Mprotect,
        vec![
            CallVar::new<MemoryAddress>("addr", In),
            CallVar::new<MemoryAddress>("len", In),
            //CallVar::new("propt", Box::new(Prot::empty()), In),
        ],
        AV::Int(Out),
    ));
     */

    inp.add(
        Ident::Munmap,
        vec!["addr", "length"],
        vec![AV::MemoryAddress(In), AV::SizeT(In)],
        AV::Int(Out),
    );
}

#[derive(Debug, PartialEq, FromPtrace, Serialize)]
#[hstrace(hmz("Protect memory {:?} - {:?} (len {}) with flags {:?}",
    self.addr,
    MemoryAddress(self.addr.0 + self.len),
    self.len,
    self.prot
))]
pub struct Mprotect {
    #[hstrace]
    pub addr: MemoryAddress,

    #[hstrace]
    pub len: usize,

    #[hstrace]
    pub prot: Prot,
}

bitflags! {
    #[derive(Serialize)]
    pub struct Prot: isize {
        const PROT_READ = 0x1;
        const PROT_WRITE = 0x2;
        const PROT_EXEC = 0x4;
        const PROT_NONE = 0x0;
        const PROT_GROWSDOWN = 0x01000000;
        const PROT_GROWSUP = 0x02000000;
    }
}
