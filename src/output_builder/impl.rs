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

    /// Sets whether to show the time.
    ///
    /// # Parameters
    /// - `show_time`: A boolean indicating whether to display the time.
    ///
    /// # Returns
    /// - `&mut Self`: A mutable reference to the struct for method chaining.
    pub fn show_time(&mut self, show_time: bool) -> &mut Self {
        self.output.show_time = show_time;
        self
    }

    /// Sets the time text color.
    ///
    /// # Parameters
    /// - `time_text_color`: The color to be set for the time text.
    ///
    /// # Returns
    /// - `&mut Self`: A mutable reference to the struct for method chaining.
    pub fn time_text_color(&mut self, time_text_color: ColorType) -> &mut Self {
        self.output.time_text_color = time_text_color;
        self
    }

    /// Sets the background color for the time text.
    ///
    /// # Parameters
    /// - `time_bg_color`: The background color to be set for the time text.
    ///
    /// # Returns
    /// - `&mut Self`: A mutable reference to the struct for method chaining.
    pub fn time_bg_color(&mut self, time_bg_color: ColorType) -> &mut Self {
        self.output.time_bg_color = time_bg_color;
        self
    }

    /// Sets whether the time text should be bold.
    ///
    /// # Parameters
    /// - `time_text_blod`: A boolean indicating whether the time text should be bold.
    ///
    /// # Returns
    /// - `&mut Self`: A mutable reference to the struct for method chaining.
    pub fn time_text_blod(&mut self, time_text_blod: bool) -> &mut Self {
        self.output.time_text_blod = time_text_blod;
        self
    }

    /// Sets the separator.
    ///
    /// # Parameters
    /// - `split`: The separator string to be set.
    ///
    /// # Returns
    /// - `&mut Self`: A mutable reference to the struct for method chaining.
    pub fn split_text(&mut self, split: &'a str) -> &mut Self {
        self.output.split = split;
        self
    }

    /// Sets the separator color.
    ///
    /// # Parameters
    /// - `split_color`: The color to be set for the separator.
    ///
    /// # Returns
    /// - `&mut Self`: A mutable reference to the struct for method chaining.
    pub fn split_text_color(&mut self, split_color: ColorType) -> &mut Self {
        self.output.split_color = split_color;
        self
    }

    /// Sets the background color for the separator.
    ///
    /// # Parameters
    /// - `split_bg_color`: The background color to be set for the separator.
    ///
    /// # Returns
    /// - `&mut Self`: A mutable reference to the struct for method chaining.
    pub fn split_bg_color(&mut self, split_bg_color: ColorType) -> &mut Self {
        self.output.split_bg_color = split_bg_color;
        self
    }

    /// Sets whether the separator text should be bold.
    ///
    /// # Parameters
    /// - `split_text_blod`: A boolean indicating whether the separator text should be bold.
    ///
    /// # Returns
    /// - `&mut Self`: A mutable reference to the struct for method chaining.
    pub fn split_text_blod(&mut self, split_text_blod: bool) -> &mut Self {
        self.output.split_text_blod = split_text_blod;
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
