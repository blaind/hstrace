/// Options for customizing the tracing behavior
#[derive(Clone, Debug)]
pub struct TraceOptions {
    pub strlen: Option<usize>,
    pub filter: FilterMode,
}

impl Default for TraceOptions {
    fn default() -> Self {
        TraceOptions {
            strlen: Some(32),
            filter: FilterMode::None,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum FilterMode {
    None,
    Files,
    Network,
    Calls(Vec<String>),
}
