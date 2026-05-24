use crate::*;

/// Configurable text display with color, background and style options.
///
/// Used for building formatted console output with various display attributes.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Text<'a> {
    /// The actual text content.
    pub text: &'a str,
    /// The color of the text.
    pub color: ColorType,
    /// The background color of the text.
    pub bg_color: ColorType,
    /// Whether the text should be bold.
    pub bold: bool,
    /// Whether to add newline after the text
    pub endl: bool,
}
