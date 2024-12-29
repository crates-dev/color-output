use crate::{output, ColorType, Output, OutputBuilder};

impl<'a> OutputBuilder<'a> {
    /// Creates the struct
    ///
    /// # Returns
    /// - `OutputBuilder`: Output
    pub fn new() -> Self {
        Self {
            output: Output::default(),
        }
    }

    /// Creates the struct from output
    ///
    /// # Returns
    /// - `OutputBuilder`: Output
    pub fn new_from(output: Output<'a>) -> Self {
        Self { output }
    }

    /// Sets the text.
    ///
    /// # Parameters
    /// - `text`: The text to be set.
    ///
    /// # Returns
    /// - `&mut Self`: A mutable reference to the struct for method chaining.
    pub fn text(&mut self, text: &'a str) -> &mut Self {
        self.output.text = text;
        self
    }

    /// Sets the text color.
    ///
    /// # Parameters
    /// - `text_color`: The color to be set for the text.
    ///
    /// # Returns
    /// - `&mut Self`: A mutable reference to the struct for method chaining.
    pub fn text_color(&mut self, text_color: ColorType) -> &mut Self {
        self.output.text_color = text_color;
        self
    }

    /// Sets the background color for the text.
    ///
    /// # Parameters
    /// - `text_bg_color`: The background color to be set for the text.
    ///
    /// # Returns
    /// - `&mut Self`: A mutable reference to the struct for method chaining.
    pub fn text_bg_color(&mut self, text_bg_color: ColorType) -> &mut Self {
        self.output.text_bg_color = text_bg_color;
        self
    }

    /// Sets whether the text should be bold.
    ///
    /// # Parameters
    /// - `text_blod`: A boolean indicating whether the text should be bold.
    ///
    /// # Returns
    /// - `&mut Self`: A mutable reference to the struct for method chaining.
    pub fn text_blod(&mut self, text_blod: bool) -> &mut Self {
        self.output.text_blod = text_blod;
        self
    }

    /// Sets the `endl` value for the `Output`.
    ///
    /// # Parameters
    /// - `endl`: A boolean value that determines whether to add a newline at the end.
    ///
    /// # Returns
    /// - `&mut Self`: Returns a mutable reference to the current `Output` instance, allowing method chaining.
    pub fn endl(&mut self, endl: bool) -> &mut Self {
        self.output.endl = endl;
        self
    }

    /// Builds and returns the Output struct.
    ///
    /// # Returns
    /// - `Output`: The constructed Output struct.
    pub fn build(&self) -> Output {
        self.output.clone()
    }

    /// Outputs the current state of the Output struct.
    ///
    /// # Returns
    /// - `()` : Nothing is returned.
    pub fn output(&self) {
        output(self.output.clone());
    }
}
