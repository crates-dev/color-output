use crate::*;

/// Represents a list of ColorOutput configurations for sequential execution.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ColorOutputList<'a>(
    /// Collection of ColorOutput configurations to execute in sequence
    pub Vec<ColorOutput<'a>>,
);
