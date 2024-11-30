use crate::{output, ColorType, Output};
/// OutputBuilder struct
///
/// [Official Documentation](https://docs.ltpp.vip/COLOR-OUTPUT/),
///
/// # Code Example
///
/// ## Using the OutputBuilder
///
/// ### Using the output Function
///
/// ```rust
/// use ltpp_output::*;
/// output(
///     OutputBuilder::new_from(Output::default())
///         .set_text("test_output_builder")
///         .set_text_color(ColorType::Color256(0xffffff))
///         .set_text_bg_color(ColorType::Color256(0xffffff))
///         .set_split_bg_color(ColorType::Color256(0xffffff))
///         .set_time_text_color(ColorType::Rgb(255, 200, 255))
///         .set_text_blod(true)
///         .set_time_text_blod(true)
///         .set_show_time(true)
///         .set_endl(true)
///         .build(),
/// );
/// ```
///
/// ### Using the output Method
///
/// ```rust
/// use ltpp_output::*;
/// OutputBuilder::new()
///     .set_text("test_output_builder_output")
///     .set_text_bg_color(ColorType::Color256(0xffffff))
///     .set_text_color(ColorType::Color256(0xffffff))
///     .set_time_text_color(ColorType::Rgb(255, 200, 255))
///     .set_text_blod(true)
///     .set_time_text_blod(true)
///     .set_show_time(true)
///     .set_endl(true)
///     .build()
///     .output();
/// ```
pub struct OutputBuilder<'a> {
    pub output: Output<'a>,
}
