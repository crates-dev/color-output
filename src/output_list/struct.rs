use crate::Output;

/// Represents a list of Output configurations for sequential execution.
///
/// [Official Documentation](https://docs.ltpp.vip/COLOR-OUTPUT/)
#[derive(Debug, Clone)]
pub struct OutputList<'a>(
    /// Collection of Output configurations to execute in sequence
    pub Vec<Output<'a>>,
);
