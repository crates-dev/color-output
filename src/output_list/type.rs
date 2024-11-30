use crate::Output;

/// OutputList struct
///
/// [Official Documentation](https://docs.ltpp.vip/LTPP-RUST-OUTPUT/),
///
/// # Code Example
///
/// ## Using the Struct
///
/// ### Using the output Method
///
/// ```rust
/// use ltpp_output::*;
/// OutputList(vec![
///     Output {
///         text: "test_output_list_struct_1",
///         text_color: ColorType::Use(Color::Default),
///         text_bg_color: ColorType::Color256(0x000000),
///         show_time: true,
///         time_text_color: ColorType::Rgb(255, 255, 255),
///         time_bg_color: ColorType::Use(Color::Yellow),
///         split: " => ",
///         split_color: ColorType::Use(Color::Cyan),
///         split_bg_color: ColorType::Use(Color::Yellow),
///         endl: false,
///         ..Default::default()
///     },
///     Output {
///         text: "test_output_struct_output_2",
///         text_color: ColorType::Use(Color::Default),
///         text_bg_color: ColorType::Use(Color::Blue),
///         show_time: true,
///         time_text_color: ColorType::Rgb(255, 255, 255),
///         time_bg_color: ColorType::Use(Color::Yellow),
///         split: " => ",
///         split_color: ColorType::Use(Color::Cyan),
///         split_bg_color: ColorType::Use(Color::Yellow),
///         endl: true,
///         ..Default::default()
///     },
/// ])
/// .output();
/// ```
#[derive(Debug, Clone)]
pub struct OutputList<'a>(pub Vec<Output<'a>>);
