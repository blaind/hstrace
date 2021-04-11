macro_rules! define_modules {
    (
        $($module_name:ident,)+
    ) => {
        $(
            mod $module_name;

            #[allow(unused_imports)]
            pub use $module_name::*;
        )+

        lazy_static! {
            pub(crate) static ref SYSCALLS: Vec<Definition<'static>> = {
                let mut definitions = Definitions::new();

                $(
                    $module_name::get_definitions(&mut definitions);
                )+

                definitions.into_definitions()
            };
        }
    };
}

macro_rules! define_calls {
    (
        $($kind:ident,)+
    ) => {
        /// Enum variant of syscall, contains call-specific data
        ///
        /// Currently only a subset of syscalls are resolved to these expanded structures
        /// Use this [GitHub issue 3](https://github.com/blaind/hstrace/issues/3) to request a new syscall implementation
        #[derive(Debug, PartialEq)]
        #[allow(non_camel_case_types)]
        pub enum SyscallKind {
            $($kind($kind),)*
            None,
        }

        impl SyscallKind {
            pub fn fmt_human(&self) -> String {
                match &self {
                    $(SyscallKind::$kind(r) => r.hmz(),)*
                    SyscallKind::None => String::from("__unknown")
                }
            }
        }

        impl From<TraceOutput> for Syscall {
            fn from(sv: TraceOutput) -> Syscall {
                let name = FromPrimitive::from_usize(sv.nr).expect("primitive conversion");

                let result = match &sv.out {
                    Some(res) => match res {
                        Ok(_) => Ok(()), // FIXME implement value
                        Err(e) => Err(e.clone()),
                    }
                    None => Ok(())
                };

                let kind: SyscallKind = match name {
                    $(
                        Ident::$kind => {
                            let x: Option<$kind> = sv.into();
                            SyscallKind::$kind(
                                match x {
                                    Some(x) => x,
                                    None => {
                                        panic!("kind conversion failed (name: {:?}), check impl From<TraceOutput> for Option<SYSCALL>", name)
                                    }
                                }
                            )
                        },
                    )*
                    _ => SyscallKind::None,
                };

                Syscall::new(name, kind, result)
            }
        }
    };
}

macro_rules! define_structs {
    (
        $($syscall:ident($kind:ty),)+
    ) => {

        #[derive(Clone)]
        #[allow(non_camel_case_types)]
        pub enum CStruct {
            $($syscall($kind),)+
        }

        impl fmt::Debug for CStruct {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    $(CStruct::$syscall(c) => write!(f, "{:?}", c),)+
                }
            }
        }

        #[derive(Clone, Debug)]
        #[allow(non_camel_case_types)]
        pub(crate) enum CStructAV {
            $($syscall,)+
        }

        impl CStructAV {
            pub fn map_value<'a, T>(&self, direction: &Direction, mut src: &mut ValueTransformer<'a, T>) -> Result<Value, TraceError>
            where
                T: crate::ptrace::Tracer,
            {
                let ret = match self {
                    $(
                        CStructAV::$syscall => {
                            if direction == &src.dir {
                                <$kind as CToCall>::from_src(&mut src)?
                            } else {
                                Value::Skip
                            }
                        },
                    )+
                };

                Ok(ret)
            }
        }
    };
}

macro_rules! define_callnames {
    (
        $($syscall:ident = $position:tt,)+
    ) => {
        #[allow(dead_code)]
        #[derive(FromPrimitive, Debug, PartialEq, Clone, Copy)]
        pub enum Ident {
            /// Unknown call, nr could not be parsed into an enum (missing implementation at hstrace)
            Unknown = -1,
            $($syscall = $position,)*
        }

        impl Ident {
            pub fn iter() -> Iter<'static, Ident> {
                // FIXME: get the number in macro invocation
                static CALLNAMES: [Ident; 347] = [
                    $(Ident::$syscall,)+
                ];

                CALLNAMES.iter()
            }
        }

        impl fmt::Display for Ident {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{:?}", self)
            }
        }
    };
}
