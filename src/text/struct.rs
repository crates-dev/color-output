use crate::*;

/// Represents a text with formatting options.
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
    /// endl
    pub(crate) endl: bool,
    /// Whether to enable automatic color contrast adjustment
    pub(crate) auto_contrast: bool,
}
