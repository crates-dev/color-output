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
///     show_time: true,
///     time_text_color: ColorType::Use(Color::Green),
///     time_bg_color: ColorType::Color256(0xffffff),
///     split: " => ",
///     split_color: ColorType::Use(Color::Cyan),
///     split_bg_color: ColorType::Use(Color::Yellow),
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
///     .time_text_color(ColorType::Use(Color::Blue))
///     .text_blod(true)
///     .time_text_blod(true)
///     .show_time(true)
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
///         show_time: true,
///         time_text_color: ColorType::Use(Color::Green),
///         time_bg_color: ColorType::Color256(0xffffff),
///         split: " => ",
///         split_color: ColorType::Use(Color::Cyan),
///         split_bg_color: ColorType::Use(Color::Yellow),
///         endl: true,
///         ..Default::default()
///     },
///     OutputBuilder::new()
///         .text("test_output_builder1")
///         .text_color(ColorType::Color256(0xffffff))
///         .time_text_color(ColorType::Rgb(255, 200, 255))
///         .text_blod(true)
///         .time_text_blod(true)
///         .show_time(true)
///         .endl(true)
///         .build(),
///     OutputBuilder::new()
///         .text("test_output_builder2")
///         .text_color(ColorType::Color256(0xffffff))
///         .time_text_color(ColorType::Rgb(255, 200, 255))
///         .text_blod(true)
///         .time_text_blod(true)
///         .show_time(true)
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
