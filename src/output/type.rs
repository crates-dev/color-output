use super::output::output;
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
///     text_color: ColorType::Use(Color::Default),
///     text_bg_color: ColorType::Color256(0x000000),
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
///     text_color: ColorType::Use(Color::Default),
///     text_bg_color: ColorType::Use(Color::Blue),
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
    pub text_color: ColorType,
    /// Text background color
    pub text_bg_color: ColorType,
    /// Bold text
    pub text_blod: bool,
    /// endl
    pub endl: bool,
}
