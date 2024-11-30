use crate::Output;

/// OutputListBuilder struct
///
/// [Official Documentation](https://docs.ltpp.vip/COLOR-OUTPUT/),
///
/// # Code Example
///
/// ## Using the OutputBuilder
///
/// ```rust
/// use ltpp_output::*;
/// OutputListBuilder::new_from(vec![Output::default()])
///     .add(
///         OutputBuilder::new()
///             .set_text("text")
///             .set_text_bg_color(ColorType::Use(Color::Blue))
///             .set_endl(false)
///             .build(),
///     )
///     .add(Output {
///         text: "test_new_from_output_list_builder_1",
///         text_color: ColorType::Use(Color::Default),
///         text_bg_color: ColorType::Color256(0x3f3f3f),
///         split: " => ",
///         split_color: ColorType::Use(Color::Cyan),
///         split_bg_color: ColorType::Use(Color::Yellow),
///         endl: false,
///         ..Default::default()
///     })
///     .add(Output {
///         text: "test_new_from_output_list_builder_2",
///         text_color: ColorType::Use(Color::Default),
///         text_bg_color: ColorType::Use(Color::Cyan),
///         split: " => ",
///         split_color: ColorType::Use(Color::Cyan),
///         split_bg_color: ColorType::Use(Color::Yellow),
///         endl: true,
///         ..Default::default()
///     })
///     .run();
/// ```
#[derive(Debug, Clone)]
pub struct OutputListBuilder<'a> {
    /// A list of output structures.
    pub output_list: Vec<Output<'a>>,
}
