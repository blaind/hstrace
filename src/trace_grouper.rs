use std::collections::HashMap;

use crate::value::Value;
use crate::{Ident, TraceOutput};

#[derive(Debug)]
pub(crate) enum ProgramState {
    Starting,
    // Started,
}

#[derive(Debug)]
pub(crate) struct StoreArg {
    pub from_arg: usize,
    pub to_hashmap: usize,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct StoreMatch {
    pub to_arg: usize,
    pub from_hashmap: usize,
}

#[derive(Debug)]
pub(crate) struct ItemCall {
    pub call: Ident,
    pub store_to: Option<StoreArg>,
    pub store_match: Option<StoreMatch>,
}

impl ItemCall {
    pub fn new(call: Ident) -> Self {
        ItemCall {
            call,
            store_to: None,
            store_match: None,
        }
    }

    pub fn new_store_to(call: Ident, store_to: StoreArg) -> Self {
        ItemCall {
            call,
            store_to: Some(store_to),
            store_match: None,
        }
    }

    pub fn new_match(call: Ident, store_match: StoreMatch) -> Self {
        ItemCall {
            call,
            store_to: None,
            store_match: Some(store_match),
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum MatchMode {
    Single,
    Multiple,
}

#[derive(Debug)]
pub(crate) struct MatchItem {
    pub mode: MatchMode,
    pub match_count: usize,
    pub calls: Vec<ItemCall>,
}

#[derive(Debug)]
pub(crate) struct Match {
    pub match_state: usize,
    pub state: ProgramState,
    pub items: Vec<MatchItem>,
    pub store: HashMap<usize, Value>,
    pub grouped: Option<GroupedIdent>,
}

#[derive(Debug)]
pub struct GroupedIdent {
    // FIXME: add file, network, etc.
    pub calls: Vec<TraceOutput>,
}

impl GroupedIdent {
    pub fn new() -> Self {
        GroupedIdent { calls: Vec::new() }
    }

    pub fn from_call(call: TraceOutput) -> Self {
        GroupedIdent { calls: vec![call] }
    }
}

pub(crate) fn get_matchers() -> Vec<Match> {
    let match_zero = StoreMatch {
        to_arg: 0,
        from_hashmap: 0,
    };

    let x = Match {
        state: ProgramState::Starting,
        store: HashMap::new(),
        match_state: 0,
        items: vec![
            MatchItem {
                mode: MatchMode::Single,
                match_count: 0,
                calls: vec![ItemCall::new_store_to(
                    Ident::Openat,
                    StoreArg {
                        from_arg: 0,
                        to_hashmap: 0,
                    },
                )],
            },
            MatchItem {
                mode: MatchMode::Multiple,
                match_count: 0,
                calls: vec![
                    ItemCall::new_match(Ident::Read, match_zero),
                    ItemCall::new_match(Ident::Fstat, match_zero),
                    ItemCall::new_match(Ident::Lseek, match_zero),
                    ItemCall::new(Ident::Mmap),
                    ItemCall::new(Ident::Mprotect),
                ],
            },
            MatchItem {
                mode: MatchMode::Single,
                match_count: 0,
                calls: vec![ItemCall::new(Ident::Close)],
            },
        ],
        grouped: Some(GroupedIdent::new()),
    };

    let x = vec![x];
    x
}
