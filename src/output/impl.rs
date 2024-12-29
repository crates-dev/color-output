use crate::{ColorType, Output};

use crate::output;

impl<'a> Default for Output<'a> {
    fn default() -> Self {
        Output {
            text: "",
            text_color: ColorType::default(),
            text_bg_color: ColorType::default(),
            text_blod: false,
            endl: false,
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
