use crate::*;

impl ColorDisplay for Color {
    fn get_str(&self, display_type: DisplayType) -> String {
        let str: &str = match display_type {
            DisplayType::Text => match self {
                Color::Red => RED,
                Color::Green => GREEN,
                Color::Blue => BLUE,
                Color::Yellow => YELLOW,
                Color::Black => BLACK,
                Color::White => WHITE,
                Color::Default => DEFAULT,
                Color::Magenta => MAGENTA,
                Color::Cyan => CYAN,
            },
            DisplayType::Background => match self {
                Color::Red => BG_RED,
                Color::Green => BG_GREEN,
                Color::Blue => BG_BLUE,
                Color::Yellow => BG_YELLOW,
                Color::Black => BG_BLACK,
                Color::White => BG_WHITE,
                Color::Default => DEFAULT,
                Color::Magenta => BG_MAGENTA,
                Color::Cyan => BG_CYAN,
            },
        };
        str.to_string()
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_str(DisplayType::Text))
    }
}

impl Display for ColorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_str(DisplayType::Text))
    }
}

impl ColorDisplay for ColorType {
    fn get_str(&self, display_type: DisplayType) -> String {
        match self {
            ColorType::Color256(fg) => match display_type {
                DisplayType::Text => color256_fg_color(*fg),
                DisplayType::Background => color256_bg_color(*fg),
            },
            ColorType::Rgb(r, g, b) => match display_type {
                DisplayType::Text => rgb_fg_color(*r, *g, *b),
                DisplayType::Background => rgb_bg_color(*r, *g, *b),
            },
            ColorType::Use(color) => color.get_str(display_type),
        }
    }
}

impl Default for ColorType {
    fn default() -> Self {
        ColorType::Use(Color::Default)
    }
}

impl ColorContrast {
    /// Calculates the relative luminance of a color.
    ///
    /// # Arguments
    ///
    /// - `u8` - Red component (0-255)
    /// - `u8` - Green component (0-255)
    /// - `u8` - Blue component (0-255)
    ///
    /// # Returns
    ///
    /// - `f64` - Relative luminance value (0.0-1.0)
    pub fn calculate_luminance(r: u8, g: u8, b: u8) -> f64 {
        let r_norm: f64 = r as f64 / 255.0;
        let g_norm: f64 = g as f64 / 255.0;
        let b_norm: f64 = b as f64 / 255.0;
        let r_linear: f64 = if r_norm <= 0.03928 {
            r_norm / 12.92
        } else {
            ((r_norm + 0.055) / 1.055).powf(2.4)
        };
        let g_linear: f64 = if g_norm <= 0.03928 {
            g_norm / 12.92
        } else {
            ((g_norm + 0.055) / 1.055).powf(2.4)
        };
        let b_linear: f64 = if b_norm <= 0.03928 {
            b_norm / 12.92
        } else {
            ((b_norm + 0.055) / 1.055).powf(2.4)
        };
        0.2126 * r_linear + 0.7152 * g_linear + 0.0722 * b_linear
    }

    /// Calculates the contrast ratio between two colors.
    ///
    /// # Arguments
    ///
    /// - `(u8, u8, u8)` - First color RGB values
    /// - `(u8, u8, u8)` - Second color RGB values
    ///
    /// # Returns
    ///
    /// - `f64` - Contrast ratio (1.0-21.0)
    pub fn calculate_contrast_ratio(color1: (u8, u8, u8), color2: (u8, u8, u8)) -> f64 {
        let lum1: f64 = Self::calculate_luminance(color1.0, color1.1, color1.2);
        let lum2: f64 = Self::calculate_luminance(color2.0, color2.1, color2.2);
        let lighter: f64 = lum1.max(lum2);
        let darker: f64 = lum1.min(lum2);
        (lighter + 0.05) / (darker + 0.05)
    }

    /// Extracts RGB values from ColorType.
    ///
    /// # Arguments
    ///
    /// - `&ColorType` - Color to extract RGB from
    ///
    /// # Returns
    ///
    /// - `(u8, u8, u8)` - RGB values
    pub fn extract_rgb_from_color_type(color: &ColorType) -> (u8, u8, u8) {
        match color {
            ColorType::Rgb(r, g, b) => (*r, *g, *b),
            ColorType::Color256(hex) => {
                let r: u8 = ((hex >> 16) & 0xFF) as u8;
                let g: u8 = ((hex >> 8) & 0xFF) as u8;
                let b: u8 = (hex & 0xFF) as u8;
                (r, g, b)
            }
            ColorType::Use(color) => {
                use super::r#enum::Color;
                match color {
                    Color::Default => (128, 128, 128),
                    Color::Black => (0, 0, 0),
                    Color::Red => (255, 0, 0),
                    Color::Green => (0, 255, 0),
                    Color::Yellow => (255, 255, 0),
                    Color::Blue => (0, 0, 255),
                    Color::Magenta => (255, 0, 255),
                    Color::Cyan => (0, 255, 255),
                    Color::White => (255, 255, 255),
                }
            }
        }
    }

    /// Checks if two colors have sufficient contrast for readability.
    ///
    /// # Arguments
    ///
    /// - `&ColorType` - Text color
    /// - `&ColorType` - Background color
    ///
    /// # Returns
    ///
    /// - `bool` - Whether contrast is sufficient (ratio >= 4.5)
    pub fn has_sufficient_contrast(text_color: &ColorType, bg_color: &ColorType) -> bool {
        let text_rgb: (u8, u8, u8) = Self::extract_rgb_from_color_type(text_color);
        let bg_rgb: (u8, u8, u8) = Self::extract_rgb_from_color_type(bg_color);
        let ratio: f64 = Self::calculate_contrast_ratio(text_rgb, bg_rgb);
        ratio >= 4.5
    }

    /// Automatically adjusts text color to ensure sufficient contrast with background.
    ///
    /// # Arguments
    ///
    /// - `&ColorType` - Original text color
    /// - `&ColorType` - Background color
    ///
    /// # Returns
    ///
    /// - `ColorType` - Adjusted text color with sufficient contrast
    pub fn ensure_sufficient_contrast(text_color: &ColorType, bg_color: &ColorType) -> ColorType {
        if Self::has_sufficient_contrast(text_color, bg_color) {
            return *text_color;
        }
        let text_rgb: (u8, u8, u8) = Self::extract_rgb_from_color_type(text_color);
        let bg_rgb: (u8, u8, u8) = Self::extract_rgb_from_color_type(bg_color);
        let bg_luminance: f64 = Self::calculate_luminance(bg_rgb.0, bg_rgb.1, bg_rgb.2);
        if bg_luminance > 0.5 {
            Self::darken_color_for_contrast(text_rgb, bg_rgb)
        } else {
            Self::lighten_color_for_contrast(text_rgb, bg_rgb)
        }
    }

    /// Darkens a color while preserving its hue for better contrast against light backgrounds.
    ///
    /// # Arguments
    ///
    /// - `(u8, u8, u8)` - Original text color RGB
    /// - `(u8, u8, u8)` - Background color RGB
    ///
    /// # Returns
    ///
    /// - `ColorType` - Darkened color with sufficient contrast
    pub fn darken_color_for_contrast(text_rgb: (u8, u8, u8), bg_rgb: (u8, u8, u8)) -> ColorType {
        let (r, g, b): (u8, u8, u8) = text_rgb;
        let max_component: u8 = r.max(g).max(b);
        if max_component == 0 {
            return ColorType::Rgb(0, 0, 0);
        }
        let scale_factor: f64 = 0.3;
        let new_r: u8 = ((r as f64 * scale_factor) as u8).min(80);
        let new_g: u8 = ((g as f64 * scale_factor) as u8).min(80);
        let new_b: u8 = ((b as f64 * scale_factor) as u8).min(80);
        let result_color: ColorType = ColorType::Rgb(new_r, new_g, new_b);
        if Self::calculate_contrast_ratio((new_r, new_g, new_b), bg_rgb) >= 4.5 {
            result_color
        } else {
            ColorType::Rgb(0, 0, 0)
        }
    }

    /// Lightens a color while preserving its hue for better contrast against dark backgrounds.
    ///
    /// # Arguments
    ///
    /// - `(u8, u8, u8)` - Original text color RGB
    /// - `(u8, u8, u8)` - Background color RGB
    ///
    /// # Returns
    ///
    /// - `ColorType` - Lightened color with sufficient contrast
    pub fn lighten_color_for_contrast(text_rgb: (u8, u8, u8), bg_rgb: (u8, u8, u8)) -> ColorType {
        let (r, g, b): (u8, u8, u8) = text_rgb;
        let scale_factor: f64 = 2.5;
        let new_r: u8 = ((r as f64 * scale_factor) as u8).max(200);
        let new_g: u8 = ((g as f64 * scale_factor) as u8).max(200);
        let new_b: u8 = ((b as f64 * scale_factor) as u8).max(200);
        let result_color: ColorType = ColorType::Rgb(new_r, new_g, new_b);
        if Self::calculate_contrast_ratio((new_r, new_g, new_b), bg_rgb) >= 4.5 {
            result_color
        } else {
            ColorType::Rgb(255, 255, 255)
        }
    }
}

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

impl<'a> Default for ColorOutputList<'a> {
    /// Provides a default implementation for ColorOutputList.
    ///
    /// # Returns
    ///
    /// - `ColorOutputList` - New instance containing a single default ColorOutput
    #[inline(always)]
    fn default() -> Self {
        ColorOutputList(vec![ColorOutput::<'a>::default()])
    }
}

impl<'a> Deref for ColorOutputList<'a> {
    type Target = Vec<ColorOutput<'a>>;

    /// Dereferences ColorOutputList to its internal Vec of ColorOutputs.
    ///
    /// # Returns
    ///
    /// - `&Vec<ColorOutput>` - Reference to the internal vector of outputs
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> IntoIterator for &'a ColorOutputList<'a> {
    type Item = &'a ColorOutput<'a>;
    type IntoIter = Iter<'a, ColorOutput<'a>>;

    /// Returns an iterator over the elements of the internal Vec.
    ///
    /// # Returns
    ///
    /// - `Iter<ColorOutput>` - Iterator over references to ColorOutput elements
    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> ColorOutputList<'a> {
    /// Provides an iterator over the elements in the internal `Vec<ColorOutput<'a>>`.
    ///
    /// # Returns
    /// - `Iter<'_, ColorOutput<'a>>`: An iterator over references to `ColorOutput` elements.
    #[inline(always)]
    pub fn iter(&self) -> std::slice::Iter<'_, ColorOutput<'a>> {
        self.0.iter()
    }

    /// ColorOutputs the content of each `ColorOutput` in the list.
    ///
    /// This method clones the `ColorOutputList` and iterates through its elements, calling the `output` method on each cloned `ColorOutput`.
    ///
    /// # Returns
    /// - `()` : Nothing is returned.
    pub fn output(self) {
        output_list(&self.to_vec());
    }
}

impl<'a> Default for ColorOutputListBuilder<'a> {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> ColorOutputListBuilder<'a> {
    /// Creates a new empty ColorOutputListBuilder.
    ///
    /// # Returns
    ///
    /// - `ColorOutputListBuilder` - New instance with empty output list
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            output_list: vec![],
        }
    }

    /// Creates a new ColorOutputListBuilder from existing outputs.
    ///
    /// # Arguments
    ///
    /// - `Vec<ColorOutput>` - Collection of outputs to initialize with
    ///
    /// # Returns
    ///
    /// - `ColorOutputListBuilder` - New instance containing the specified outputs
    #[inline(always)]
    pub fn new_from(output_list: Vec<ColorOutput<'a>>) -> Self {
        Self { output_list }
    }

    /// Adds an output to the list.
    ///
    /// # Arguments
    ///
    /// - `ColorOutput` - The output configuration to add
    ///
    /// # Returns
    ///
    /// - `&mut Self` - The builder for method chaining
    #[inline(always)]
    pub fn add(&mut self, output: ColorOutput<'a>) -> &mut Self {
        self.output_list.push(output);
        self
    }

    /// Removes an output item from the list at the specified index.
    ///
    /// # Parameters
    /// - `&mut self`: A mutable reference to the current instance of `ColorOutputListBuilder`.
    /// - `idx`: The index of the output item to be removed.
    ///
    /// # Returns
    /// - `&mut Self`: A mutable reference to the current instance, allowing for method chaining.
    ///
    /// If the index is out of bounds, the list remains unchanged.
    pub fn remove(&mut self, idx: usize) -> &mut Self {
        if idx >= self.output_list.len() {
            return self;
        }
        self.output_list.remove(idx);
        self
    }

    /// Clears all output items from the output list.
    ///
    /// # Parameters
    /// - `&mut self`: A mutable reference to the current instance of `ColorOutputListBuilder`.
    #[inline(always)]
    pub fn clear(&mut self) {
        self.output_list.clear();
    }

    /// Runs all output items in the list, executing their output logic.
    ///
    /// # Parameters
    /// - `&mut self`: A mutable reference to the current instance of `ColorOutputListBuilder`.
    ///
    /// # Returns
    /// - `&mut Self`: A mutable reference to the current instance, allowing for method chaining.
    ///
    /// The method clones the current output list, clears the original list, and executes
    /// the output for each cloned item.
    pub fn run(&mut self) -> &mut Self {
        let outputs: Vec<ColorOutput<'_>> = self.output_list.to_vec();
        self.clear();
        output_list(&outputs);
        self
    }

    /// Queries the output item at the specified index.
    ///
    /// # Parameters
    /// - `&self`: An immutable reference to the current instance of `ColorOutputListBuilder`.
    /// - `idx`: The index of the output item to query.
    ///
    /// # Returns
    /// - `ColorOutput`: The output item at the specified index, or a default output if the index is out of bounds.
    pub fn query_idx(&'_ self, idx: usize) -> ColorOutput<'_> {
        if idx >= self.output_list.len() {
            return ColorOutput::default();
        }
        self.output_list[idx]
    }

    /// Runs the output item at the specified index.
    ///
    /// # Parameters
    /// - `&mut self`: A mutable reference to the current instance of `ColorOutputListBuilder`.
    /// - `idx`: The index of the output item to run.
    ///
    /// # Returns
    /// - `&mut Self`: A mutable reference to the current instance, allowing for method chaining.
    ///
    /// If the index is out of bounds, the list remains unchanged.
    pub fn run_idx(&mut self, idx: usize) -> &mut Self {
        if idx >= self.output_list.len() {
            return self;
        }
        let output: ColorOutput<'_> = self.query_idx(idx);
        output.output();
        self
    }
}
