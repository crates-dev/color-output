use crate::*;

/// Builder pattern for constructing Output configurations.
///
/// [Official Documentation](https://docs.ltpp.vip/COLOR-OUTPUT/)
pub struct OutputBuilder<'a> {
    /// The Output configuration being built.
    pub output: Output<'a>,
}
