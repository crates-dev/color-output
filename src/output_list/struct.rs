use crate::*;

/// Represents a list of Output configurations for sequential execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputList<'a>(
    /// Collection of Output configurations to execute in sequence
    pub Vec<Output<'a>>,
);
