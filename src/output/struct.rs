use crate::*;

/// Represents a colored text output with formatting options.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Output<'a> {
    /// The text content to output.
    pub text: &'a str,
    /// The text color.
    pub color: ColorType,
    /// The background color.
    pub bg_color: ColorType,
    /// Whether the text should be bold.
    pub blod: bool,
    /// Whether to add a newline after the text.
    pub endl: bool,
}
