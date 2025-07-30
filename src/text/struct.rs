use crate::*;

/// Configurable text display with color, background and style options.
///
/// Used for building formatted console output with various display attributes.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Text<'a> {
    /// The actual text content.
    pub(crate) text: &'a str,
    /// The color of the text.
    pub(crate) color: ColorType,
    /// The background color of the text.
    pub(crate) bg_color: ColorType,
    /// Whether the text should be bold.
    pub(crate) blod: bool,
    /// Whether to add newline after the text
    pub(crate) endl: bool,
}
