use crate::value::map_output;
use crate::value::Value;
use crate::Ident;
use crate::SyscallParameters;
use crate::{Definition, SyscallError};
use colored::Colorize;
use std::fmt;

/// This will format line
pub struct TraceOutput {
    /// Process id where syscall occured
    pub pid: usize,

    /// Syscall index number for current kernel
    pub nr: usize,

    /// Syscall name, if it was resolved
    pub ident: Ident,

    /// In-out variables
    pub variables: Vec<Value>,

    /// Out return code
    pub out: Option<Result<Value, SyscallError>>,
}

impl TraceOutput {
    pub(crate) fn new(ident: Ident, nr: usize, variables: Vec<Value>, pid: usize) -> Self {
        TraceOutput {
            ident,
            nr,
            variables,
            out: None,
            pid,
        }
    }

    pub(crate) fn from_definition<T>(
        ptrace: &mut T,
        definition: &Definition,
        info: &SyscallParameters,
    ) -> Self
    where
        T: crate::Tracer,
    {
        let mut variables: Vec<Value> = Vec::new();

        // FIXME this initialization could be more performant, but type should be Vec<Option<Value>>, notice rev below
        for _ in 0..definition.var_types.len() {
            variables.push(Value::None);
        }

        for (idx, var_type) in definition.var_types.iter().enumerate().rev() {
            let mapped_val = info.transform_value(idx, ptrace, var_type, &variables);

            variables[idx] = mapped_val;
        }

        TraceOutput::new(definition.ident, info.nr, variables, info.pid)
    }

    pub(crate) fn update_exit<T>(&mut self, ptrace: &mut T, info: &SyscallParameters)
    where
        T: crate::Tracer,
    {
        let definition = info.get_definition().unwrap();

        // set output
        self.out = Some(map_output(
            &definition.out_value,
            info.exit.is_error,
            info.exit.rval,
        ));

        // set variables
        let mut updated_variables: Vec<Option<Value>> = Vec::with_capacity(self.variables.len());

        for idx in 0..self.variables.len() {
            let val =
                info.transform_value(idx, ptrace, &definition.var_types[idx], &self.variables);

            if let Value::Skip = val {
                // skip this, was input value
                updated_variables.push(None);
            } else {
                //*out = val;
                updated_variables.push(Some(val));
            }
        }

        for (idx, transformed_input) in updated_variables.into_iter().enumerate() {
            if let Some(input) = transformed_input {
                self.variables[idx] = input;
            }
        }
    }
}

impl fmt::Debug for TraceOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let var_strs: Vec<String> = self
            .variables
            .iter()
            .map(|a| format!("{}", format!("{:?}", a).cyan()))
            .collect();

        let ident_str = match &self.ident {
            Ident::Unknown => format!("__unknown"),
            _ => format!("{}", self.ident.to_string().to_lowercase()),
        }
        .yellow();

        let out_var = match self.out.as_ref() {
            Some(o) => match o {
                Ok(o) => format!("{:?}", o).green(),
                Err(e) => format!("-1 {:?} ({})", e, e.to_errno().desc()).magenta(),
            },
            None => format!("?").yellow(),
        };

        write!(
            f,
            "[{}] {}{}{}{} = {}", // [pid] ...
            self.pid,
            ident_str,
            "(".purple(),
            var_strs.join(", "),
            ")".purple(),
            out_var,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_output_debug() {
        let output = TraceOutput::new(
            Ident::Brk,
            Ident::Brk as usize,
            vec![Value::Int(50), Value::Int(10)],
            50,
        );

        colored::control::set_override(false);
        assert_eq!(format!("{:?}", output), "[50] brk(50, 10) = ?");

        colored::control::set_override(true);
        assert_eq!(format!("{:?}", output), "[50] \u{1b}[33mbrk\u{1b}[0m\u{1b}[35m(\u{1b}[0m\u{1b}[36m50\u{1b}[0m, \u{1b}[36m10\u{1b}[0m\u{1b}[35m)\u{1b}[0m = \u{1b}[33m?\u{1b}[0m");
    }
}
