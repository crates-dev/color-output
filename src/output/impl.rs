use crate::*;

/// Default implementation for Output with empty configuration.
impl<'a> Default for Output<'a> {
    #[inline(always)]
    fn default() -> Self {
        Output {
            text: "",
            color: ColorType::default(),
            bg_color: ColorType::default(),
            bold: false,
            endl: false,
        }
    }
}

impl<'a> Output<'a> {
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
