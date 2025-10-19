use crate::*;

/// Represents different types of colors that can be used for text formatting.
#[derive(Debug, Clone, PartialEq)]
pub enum ColorType {
    /// RGB color with red, green and blue components.
    Rgb(u8, u8, u8),
    /// 256-color palette color.
    Color256(u32),
    /// Predefined built-in colors.
    Use(Color),
}

/// Specifies whether color applies to text or background.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum DisplayType {
    /// Color applies to text.
    Text,
    /// Color applies to background.
    Background,
}

/// Predefined color constants for easy text formatting.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum Color {
    /// Default terminal color.
    #[default]
    Default,
    /// Black color.
    Black,
    /// Red color.
    Red,
    /// Green color.
    Green,
    /// Yellow color.
    Yellow,
    /// Blue color.
    Blue,
    /// Magenta color.
    Magenta,
    /// Cyan color.
    Cyan,
    /// White color.
    White,
}

/// Trait for converting colors to their ANSI escape sequences.
pub(crate) trait ColorDisplay {
    /// Gets the ANSI escape sequence for the color.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Reference to self
    /// - `DisplayType` - Whether to apply to text or background
    ///
    /// # Returns
    ///
    /// - `String` - The ANSI escape sequence string
    fn get_str(&self, display_type: DisplayType) -> String;
}

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
            ColorType::Use(color) => color.get_str(display_type.clone()),
        }
    }
}

impl Default for ColorType {
    fn default() -> Self {
        ColorType::Use(Color::Default)
    }
}
