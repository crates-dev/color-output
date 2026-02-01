use crate::*;

/// Builder pattern for constructing OutputList configurations.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct OutputListBuilder<'a> {
    /// Collection of Output configurations being built.
    pub output_list: Vec<Output<'a>>,
}
