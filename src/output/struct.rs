use crate::*;

/// Output struct
///
/// [Official Documentation](https://docs.ltpp.vip/COLOR-OUTPUT/),
///
/// # Code Example
///
/// ## Using the Struct
///
/// ### Using the output Function
///
/// ```rust
/// use color_output::*;
/// output(Output {
///     text: "test_output_struct",
///     color: ColorType::Use(Color::Default),
///     bg_color: ColorType::Color256(0x000000),
///     endl: true,
///     ..Default::default()
/// });
/// ```
///
/// ### Using the output Method
///
/// ```rust
/// use color_output::*;
/// Output {
///     text: "test_output_struct_output",
///     color: ColorType::Use(Color::Default),
///     bg_color: ColorType::Use(Color::Blue),
///     endl: true,
///     ..Default::default()
/// }
/// .output();
/// ```
#[derive(Debug, Clone)]
pub struct Output<'a> {
    /// text
    pub text: &'a str,
    /// text color
    pub color: ColorType,
    /// Text background color
    pub bg_color: ColorType,
    /// Bold text
    pub blod: bool,
    /// endl
    pub endl: bool,
    /// Whether to enable automatic color contrast adjustment
    pub auto_contrast: bool,
}
