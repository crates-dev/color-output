use crate::*;

/// Default implementation for ColorOutput with empty configuration.
impl<'a> Default for ColorOutput<'a> {
    #[inline(always)]
    fn default() -> Self {
        ColorOutput {
            text: "",
            color: ColorType::default(),
            bg_color: ColorType::default(),
            bold: false,
            endl: false,
        }
    }
}

impl<'a> ColorOutput<'a> {
    /// Executes the output operation with current configuration.
    ///
    /// # Returns
    ///
    /// - `()` - No return value
    #[inline(always)]
    pub fn output(self) {
        output(self);
    }
}
