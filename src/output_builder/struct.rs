use crate::*;

/// Builder pattern for constructing Output configurations.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct OutputBuilder<'a> {
    /// The Output configuration being built.
    pub output: Output<'a>,
}
