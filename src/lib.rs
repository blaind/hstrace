pub mod prelude;

/// Tracer
///
/// Usage:
/// ```
/// use hstrace::prelude::*;
/// let tracer = HStrace::new();
/// ```
pub struct HStrace {}

impl HStrace {
    /// Constructs a new tracer
    pub fn new() -> Self {
        HStrace {}
    }
}
