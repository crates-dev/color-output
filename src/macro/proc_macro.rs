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
/// use ltpp_output::*;
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
/// use ltpp_output::*;
/// output_macro!(OutputBuilder::new()
///     .set_text("test_output_builder")
///     .set_text_color(ColorType::Use(Color::Cyan))
///     .set_time_text_color(ColorType::Use(Color::Blue))
///     .set_text_blod(true)
///     .set_time_text_blod(true)
///     .set_show_time(true)
///     .set_endl(true)
///     .build());
/// ```
///
/// ## Multiple Inputs
///
/// ```rust
/// use ltpp_output::*;
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
///         .set_text("test_output_builder1")
///         .set_text_color(ColorType::Color256(0xffffff))
///         .set_time_text_color(ColorType::Rgb(255, 200, 255))
///         .set_text_blod(true)
///         .set_time_text_blod(true)
///         .set_show_time(true)
///         .set_endl(true)
///         .build(),
///     OutputBuilder::new()
///         .set_text("test_output_builder2")
///         .set_text_color(ColorType::Color256(0xffffff))
///         .set_time_text_color(ColorType::Rgb(255, 200, 255))
///         .set_text_blod(true)
///         .set_time_text_blod(true)
///         .set_show_time(true)
///         .set_endl(true)
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
