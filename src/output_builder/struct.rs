use crate::*;

/// Builder pattern for constructing Output configurations.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct OutputBuilder<'a> {
    /// The Output configuration being built.
    pub output: Output<'a>,
}
