use crate::*;

/// Builder pattern for constructing ColorOutput configurations.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ColorOutputBuilder<'a> {
    /// The ColorOutput configuration being built.
    pub output: ColorOutput<'a>,
}
