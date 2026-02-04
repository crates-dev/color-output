use crate::*;

/// Builder pattern for constructing ColorOutputList configurations.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ColorOutputListBuilder<'a> {
    /// Collection of ColorOutput configurations being built.
    pub output_list: Vec<ColorOutput<'a>>,
}
