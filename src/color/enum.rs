/// Represents different types of colors that can be used for text formatting.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ColorType {
    /// RGB color with red, green and blue components.
    Rgb(u8, u8, u8),
    /// 256-color palette color.
    Color256(u32),
    /// Predefined built-in colors.
    Use(Color),
}

/// Specifies whether color applies to text or background.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub(crate) enum DisplayType {
    /// Color applies to text.
    #[default]
    Text,
    /// Color applies to background.
    Background,
}

/// Predefined color constants for easy text formatting.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
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
