use super::prelude::*;

pub(crate) fn get_definitions(inp: &mut Definitions) {
    inp.add(
        Ident::Clone,
        vec![],
        vec![], // FIXME TODO!
        AV::Int(Out),
    );
}
