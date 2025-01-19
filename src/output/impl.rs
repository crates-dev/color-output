use crate::*;

impl<'a> Default for Output<'a> {
    #[inline]
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
    /// Outputs
    ///
    /// # Returns
    /// - `()` : Nothing is returned.
    #[inline]
    pub fn output(self) {
        output(self);
    }
}
