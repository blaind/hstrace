use crate::value::{Value, AV};
use std::fmt::{self, Debug};

mod error;
pub use error::*;

#[cfg(target_arch = "x86_64")]
mod x86_64;
#[cfg(target_arch = "x86_64")]
pub use x86_64::*;

#[derive(PartialEq, Clone, Debug, Copy)]
pub(crate) enum Direction {
    In,
    Out,
    InOut,
}

/// Defines a syscall mapping
pub(crate) struct Definition<'a> {
    pub call_nr: usize,
    pub call_name: String,
    pub ident: Ident,
    #[allow(dead_code)]
    pub var_names: Vec<&'a str>,
    pub var_types: Vec<AV>,
    pub out_value: AV,
}

impl<'a> Definition<'a> {
    pub fn new(ident: Ident, var_names: Vec<&'a str>, var_types: Vec<AV>, out_value: AV) -> Self {
        let call_nr = ident.clone() as usize;
        let call_name = ident.to_string().to_lowercase();

        Definition {
            call_nr,
            call_name,
            ident,
            var_names,
            var_types,
            out_value,
        }
    }
}

impl<'a> fmt::Debug for Definition<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Definition({:?}, nr={})", self.call_name, self.call_nr)
    }
}

pub(crate) struct Definitions<'a> {
    definitions: Vec<Definition<'a>>,
}

impl<'a> Definitions<'a> {
    pub fn new() -> Self {
        Self {
            definitions: Vec::new(),
        }
    }

    pub fn add(&mut self, name: Ident, inp_f: Vec<&'a str>, inp: Vec<AV>, out: AV) {
        self.push(Definition::new(name, inp_f, inp, out))
    }

    pub fn push(&mut self, definition: Definition<'a>) {
        self.definitions.push(definition);
    }

    pub fn into_definitions(self) -> Vec<Definition<'a>> {
        self.definitions
    }
}

/*
/// Defines a syscall mapping
pub(crate) struct NewDefinition<T> {
    pub nr: usize,
    pub name: String,
    pub vars: Vec<Box<CallVar<T>>>,
    pub out: AV,
}

impl NewDefinition {
    pub fn new(name: Ident, vars: Vec<CallVar<T>>, out: AV) -> Self {
        NewDefinition {
            nr: name.clone() as usize,
            name: name.to_string().to_lowercase(),
            vars,
            out,
        }
    }
}

impl fmt::Debug for NewDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Definition({:?}, nr={})", self.name, self.nr)
    }
}

/// A syscall variable
pub(crate) struct CallVar<T> {
    name: String,
    direction: Direction,
    _phantom: PhantomData<T>,
}

impl<T> CallVar<T> {
    pub fn new(name: &'static str, direction: Direction) -> Self
    where
        T: VarType,
    {
        Self {
            name: name.to_owned(),
            direction,
            _phantom: PhantomData,
        }
    }
}

pub(crate) trait VarType {
    /*
    fn to_value<'a, T>(self: &Self, vt: &'a mut ValueTransformer<T>) -> Result<&'a str, TraceError>
    where
        T: crate::Ptrace;
         */

    fn to_value<T>(self: &Self, x: ValueTransformer<T>) -> Result<String, TraceError>
    where
        T: Ptrace;
}

 */

pub trait VarType: Send + Sync {
    fn convert(&self, input: u64) -> Value;
}

pub trait VarOutType: Send + Sync + Debug {}
