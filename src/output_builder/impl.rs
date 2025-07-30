use crate::*;

/// Implementation of OutputBuilder methods.
impl<'a> OutputBuilder<'a> {
    /// Creates a new OutputBuilder instance.
    ///
    /// # Returns
    ///
    /// - `OutputBuilder<'a>` - The new builder instance.
    pub fn new() -> Self {
        Self {
            output: Output::default(),
        }
    }

    /// Creates a new OutputBuilder from existing Output.
    ///
    /// # Arguments
    ///
    /// - `Output` - The output configuration to initialize from
    ///
    /// # Returns
    ///
    /// - `OutputBuilder` - The new builder instance
    pub fn new_from(output: Output<'a>) -> Self {
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
    pub fn blod(&mut self, blod: bool) -> &mut Self {
        self.output.blod = blod;
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
    pub fn endl(&mut self, endl: bool) -> &mut Self {
        self.output.endl = endl;
        self
    }

    /// Builds the final Output.
    ///
    /// # Returns
    ///
    /// - `Output<'a>` - The constructed output.
    pub fn build(&self) -> Output {
        self.output.clone()
    }

    /// Outputs the current state.
    ///
    /// # Returns
    ///
    /// - `()` - No return value.
    pub fn output(&self) {
        output(self.output.clone());
    }
}
