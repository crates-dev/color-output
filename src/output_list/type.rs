use crate::Output;

/// OutputList struct
///
/// [Official Documentation](https://docs.ltpp.vip/COLOR-OUTPUT/),
///
/// # Code Example
///
/// ## Using the Struct
///
/// ### Using the output Method
///
/// ```rust
/// use color_output::*;
/// OutputList(vec![
///     Output {
///         text: "test_output_list_struct_1",
///         text_color: ColorType::Use(Color::Default),
///         text_bg_color: ColorType::Color256(0x000000),
///         endl: false,
///         ..Default::default()
///     },
///     Output {
///         text: "test_output_struct_output_2",
///         text_color: ColorType::Use(Color::Default),
///         text_bg_color: ColorType::Use(Color::Blue),
///         endl: true,
///         ..Default::default()
///     },
/// ])
/// .output();
/// ```
#[derive(Debug, Clone)]
pub struct OutputList<'a>(pub Vec<Output<'a>>);
