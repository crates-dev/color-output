use crate::*;

/// Default implementation for Output with empty configuration.
impl<'a> Default for Output<'a> {
    fn default() -> Self {
        Output {
            text: "",
            color: ColorType::default(),
            bg_color: ColorType::default(),
            blod: false,
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
    pub fn output(self) {
        output(self);
    }
}
