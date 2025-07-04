use crate::*;

impl<'a> Default for Output<'a> {
    fn default() -> Self {
        Output {
            text: "",
            color: ColorType::default(),
            bg_color: ColorType::default(),
            blod: false,
            endl: false,
            auto_contrast: true,
        }
    }
}

impl<'a> Output<'a> {
    /// Outputs
    ///
    /// # Returns
    /// - `()` : Nothing is returned.
    pub fn output(self) {
        output(self);
    }
}
