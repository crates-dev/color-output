use crate::*;

/// Builder pattern for constructing OutputList configurations.
///
/// [Official Documentation](https://docs.ltpp.vip/COLOR-OUTPUT/)
#[derive(Debug, Clone)]
pub struct OutputListBuilder<'a> {
    /// Collection of Output configurations being built.
    pub output_list: Vec<Output<'a>>,
}
