use super::prelude::*;

pub(crate) fn get_definitions(inp: &mut Definitions) {
    inp.add(
        Ident::ArchPrctl,
        vec!["code", "addr"],
        // FIXME addr direction depends on the "code"
        vec![AV::MemoryAddress(In), AV::MemoryAddress(InOut)],
        AV::Int(Out),
    );
}
