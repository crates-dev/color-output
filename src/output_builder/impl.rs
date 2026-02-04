use crate::*;

/// Implementation of ColorOutputBuilder methods.
impl<'a> Default for ColorOutputBuilder<'a> {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ColorOutputBuilder<'a> {
    /// Creates a new ColorOutputBuilder instance.
    ///
    /// # Returns
    ///
    /// - `ColorOutputBuilder<'a>` - The new builder instance.
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            output: ColorOutput::default(),
        }
    }

    /// Creates a new ColorOutputBuilder from existing ColorOutput.
    ///
    /// # Arguments
    ///
    /// - `ColorOutput` - The output configuration to initialize from
    ///
    /// # Returns
    ///
    /// - `ColorOutputBuilder` - The new builder instance
    #[inline(always)]
    pub fn new_from(output: ColorOutput<'a>) -> Self {
        Self { output }
    }

    /// Sets the output text.
    ///
    /// # Arguments
    ///
    /// - `&str` - The text content to display
    ///
    /// # Returns
    ///
    /// - `&mut Self` - The builder for method chaining
    #[inline(always)]
    pub fn text(&mut self, text: &'a str) -> &mut Self {
        self.output.text = text;
        self
    }

    /// Sets the text color.
    ///
    /// # Arguments
    ///
    /// - `ColorType` - The color to apply to text
    ///
    /// # Returns
    ///
    /// - `&mut Self` - The builder for method chaining
    #[inline(always)]
    pub fn color(&mut self, color: ColorType) -> &mut Self {
        self.output.color = color;
        self
    }

    /// Sets the background color.
    ///
    /// # Arguments
    ///
    /// - `ColorType` - The background color type.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - The builder for chaining.
    #[inline(always)]
    pub fn bg_color(&mut self, bg_color: ColorType) -> &mut Self {
        self.output.bg_color = bg_color;
        self
    }

    /// Sets bold text style.
    ///
    /// # Arguments
    ///
    /// - `bool` - Whether to use bold style.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - The builder for chaining.
    #[inline(always)]
    pub fn bold(&mut self, bold: bool) -> &mut Self {
        self.output.bold = bold;
        self
    }

    /// Sets whether to add newline at end.
    ///
    /// # Arguments
    ///
    /// - `bool` - Whether to add newline.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - The builder for chaining.
    #[inline(always)]
    pub fn endl(&mut self, endl: bool) -> &mut Self {
        self.output.endl = endl;
        self
    }

    /// Builds the final ColorOutput.
    ///
    /// # Returns
    ///
    /// - `ColorOutput<'a>` - The constructed output.
    #[inline(always)]
    pub fn build(&'_ self) -> ColorOutput<'_> {
        self.output
    }

    /// ColorOutputs the current state.
    ///
    /// # Returns
    ///
    /// - `()` - No return value.
    #[inline(always)]
    pub fn output(&self) {
        output(self.output);
    }
}
