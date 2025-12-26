use crate::*;

/// Builder pattern for constructing OutputList configurations.
#[derive(Debug, Clone)]
pub struct OutputListBuilder<'a> {
    /// Collection of Output configurations being built.
    pub output_list: Vec<Output<'a>>,
}
