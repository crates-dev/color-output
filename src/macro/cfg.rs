use crate::*;

#[test]
fn test_proc_macro_output_struct() {
    output_macro!(Output {
        text: "test_proc_macro",
        text_color: ColorType::default(),
        text_bg_color: ColorType::Use(Color::Yellow),
        show_time: true,
        time_text_color: ColorType::Use(Color::Green),
        time_bg_color: ColorType::Color256(0xffffff),
        split: " => ",
        split_color: ColorType::Use(Color::Cyan),
        split_bg_color: ColorType::Use(Color::Yellow),
        endl: true,
        ..Default::default()
    });
}

#[test]
fn test_proc_mcacro_output_builder() {
    output_macro!(OutputBuilder::new()
        .text("test_output_builder")
        .text_color(ColorType::Use(Color::Cyan))
        .time_text_color(ColorType::Use(Color::Blue))
        .text_blod(true)
        .time_text_blod(true)
        .show_time(true)
        .endl(true)
        .build());
}

#[test]
fn test_proc_macro_multiple() {
    output_macro!(
        Output {
            text: "test_proc_macro",
            text_color: ColorType::default(),
            text_bg_color: ColorType::Use(Color::Yellow),
            show_time: true,
            time_text_color: ColorType::Use(Color::Green),
            time_bg_color: ColorType::Color256(0xffffff),
            split: " => ",
            split_color: ColorType::Use(Color::Cyan),
            split_bg_color: ColorType::Use(Color::Yellow),
            endl: true,
            ..Default::default()
        },
        OutputBuilder::new()
            .text("test_output_builder1")
            .text_color(ColorType::Color256(0xffffff))
            .time_text_color(ColorType::Rgb(255, 200, 255))
            .text_blod(true)
            .time_text_blod(true)
            .show_time(true)
            .endl(true)
            .build(),
        OutputBuilder::new()
            .text("test_output_builder2")
            .text_color(ColorType::Color256(0xffffff))
            .time_text_color(ColorType::Rgb(255, 200, 255))
            .text_blod(true)
            .time_text_blod(true)
            .show_time(true)
            .endl(true)
            .build()
    );
}
