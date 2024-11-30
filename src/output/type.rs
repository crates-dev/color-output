use super::output::output;
use crate::*;

/// Output struct
///
/// [Official Documentation](https://docs.ltpp.vip/LTPP-RUST-OUTPUT/),
///
/// # Code Example
///
/// ## Using the Struct
///
/// ### Using the output Function
///
/// ```rust
/// use ltpp_output::*;
/// output(Output {
///     text: "test_output_struct",
///     text_color: ColorType::Use(Color::Default),
///     text_bg_color: ColorType::Color256(0x000000),
///     show_time: true,
///     time_text_color: ColorType::Rgb(255, 255, 255),
///     time_bg_color: ColorType::Use(Color::Yellow),
///     split: " => ",
///     split_color: ColorType::Use(Color::Cyan),
///     split_bg_color: ColorType::Use(Color::Yellow),
///     endl: true,
///     ..Default::default()
/// });
/// ```
///
/// ### Using the output Method
///
/// ```rust
/// use ltpp_output::*;
/// Output {
///     text: "test_output_struct_output",
///     text_color: ColorType::Use(Color::Default),
///     text_bg_color: ColorType::Use(Color::Blue),
///     show_time: true,
///     time_text_color: ColorType::Rgb(255, 255, 255),
///     time_bg_color: ColorType::Use(Color::Yellow),
///     split: " => ",
///     split_color: ColorType::Use(Color::Cyan),
///     split_bg_color: ColorType::Use(Color::Yellow),
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
    /// Whether to show time
    pub show_time: bool,
    /// Time text color
    pub time_text_color: ColorType,
    /// Time background color
    pub time_bg_color: ColorType,
    /// Time text bold
    pub time_text_blod: bool,
    /// Separator
    pub split: &'a str,
    /// Separator text color
    pub split_color: ColorType,
    /// Separator background color
    pub split_bg_color: ColorType,
    /// Separator text bold
    pub split_text_blod: bool,
    /// endl
    pub endl: bool,
}
