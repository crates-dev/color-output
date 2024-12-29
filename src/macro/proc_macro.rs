/// Output macro
///
/// [Official Documentation](https://docs.ltpp.vip/COLOR-OUTPUT/),
///
/// # Parameters
/// - `Output`: The output struct
///
/// # Code Example
///
/// ## Using the Struct
///
/// ```rust
/// use color_output::*;
/// output_macro!(Output {
///     text: "test_proc_macro",
///     text_color: ColorType::default(),
///     text_bg_color: ColorType::Use(Color::Yellow),
///     endl: true,
///     ..Default::default()
/// });
/// ```
///
/// ## Using the Constructor
///
/// ```rust
/// use color_output::*;
/// output_macro!(OutputBuilder::new()
///     .text("test_output_builder")
///     .text_color(ColorType::Use(Color::Cyan))
///     .text_blod(true)
///     .endl(true)
///     .build());
/// ```
///
/// ## Multiple Inputs
///
/// ```rust
/// use color_output::*;
/// output_macro!(
///     Output {
///         text: "test_proc_macro",
///         text_color: ColorType::default(),
///         text_bg_color: ColorType::Use(Color::Yellow),
///         endl: true,
///         ..Default::default()
///     },
///     OutputBuilder::new()
///         .text("test_output_builder1")
///         .text_color(ColorType::Color256(0xffffff))
///         .text_blod(true)
///         .endl(true)
///         .build(),
///     OutputBuilder::new()
///         .text("test_output_builder2")
///         .text_color(ColorType::Color256(0xffffff))
///         .text_blod(true)
///         .endl(true)
///         .build()
/// );
/// ```
#[macro_export]
macro_rules! output_macro {
    ($($output:expr),*) => {
        $(
            $output.output();
        )*
    };
}
