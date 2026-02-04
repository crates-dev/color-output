use crate::*;

/// Provides utilities for calculating and ensuring color contrast ratios.
///
/// Used to determine if text colors meet WCAG accessibility standards
/// when displayed on specific background colors.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ColorContrast;

/// Represents a colored text output with formatting options.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ColorOutput<'a> {
    /// The text content to output.
    pub text: &'a str,
    /// The text color.
    pub color: ColorType,
    /// The background color.
    pub bg_color: ColorType,
    /// Whether the text should be bold.
    pub bold: bool,
    /// Whether to add a newline after the text.
    pub endl: bool,
}

/// Builder pattern for constructing ColorOutput configurations.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ColorOutputBuilder<'a> {
    /// The ColorOutput configuration being built.
    pub output: ColorOutput<'a>,
}

/// Represents a list of ColorOutput configurations for sequential execution.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ColorOutputList<'a>(
    /// Collection of ColorOutput configurations to execute in sequence
    pub Vec<ColorOutput<'a>>,
);

/// Builder pattern for constructing ColorOutputList configurations.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ColorOutputListBuilder<'a> {
    /// Collection of ColorOutput configurations being built.
    pub output_list: Vec<ColorOutput<'a>>,
}
