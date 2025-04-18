use super::r#fn::{color256_bg_color, color256_fg_color, rgb_bg_color, rgb_fg_color};
use crate::color::r#const::{
    BG_BLACK, BG_BLUE, BG_CYAN, BG_GREEN, BG_MAGENTA, BG_RED, BG_WHITE, BG_YELLOW, BLACK, BLUE,
    CYAN, DEFAULT, GREEN, MAGENTA, RED, WHITE, YELLOW,
};
use std::fmt::{self, Display};

/// ColorType
#[derive(Debug, Clone, PartialEq)]
pub enum ColorType {
    /// RGB Color (r, g, b),
    Rgb(u8, u8, u8),
    /// Color 256
    Color256(u32),
    /// Built-in Colors
    Use(Color),
}

/// Display Type
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum DisplayType {
    /// Text
    Text,
    /// Background
    Background,
}

/// Color
#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    /// Default Color
    Default,
    /// Black
    Black,
    /// Red
    Red,
    /// Green
    Green,
    /// Yellow
    Yellow,
    /// Blue
    Blue,
    /// Magenta
    Magenta,
    /// Cyan
    Cyan,
    /// White
    White,
}

pub(crate) trait ColorDisplay {
    /// Gets the display string
    ///
    /// # Parameters
    /// - `&Self`: &self
    /// - `DisplayType`: display_type
    ///
    /// # Returns
    /// - `String`: The displayed string
    fn get_str(&self, display_type: DisplayType) -> String;
}

impl Default for Color {
    fn default() -> Self {
        Color::Default
    }
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
