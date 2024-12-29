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
/// use color_output::*;
/// output(
///     OutputBuilder::new_from(Output::default())
///         .text("test_output_builder")
///         .color(ColorType::Color256(0xffffff))
///         .bg_color(ColorType::Color256(0xffffff))
///         .blod(true)
///         .endl(true)
///         .build(),
/// );
/// ```
///
/// ### Using the output Method
///
/// ```rust
/// use color_output::*;
/// OutputBuilder::new()
///     .text("test_output_builder_output")
///     .bg_color(ColorType::Color256(0xffffff))
///     .color(ColorType::Color256(0xffffff))
///     .blod(true)
///     .endl(true)
///     .build()
///     .output();
/// ```
pub struct OutputBuilder<'a> {
    pub output: Output<'a>,
}
