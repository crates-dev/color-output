use crate::*;
#[test]
fn test_output_builder_new_from() {
    output(
        OutputBuilder::new()
            .text("test_output_builder")
            .text_color(ColorType::Color256(0xffffff))
            .text_bg_color(ColorType::Color256(0xffffff))
            .split_bg_color(ColorType::Color256(0xffffff))
            .time_text_color(ColorType::Rgb(255, 200, 255))
            .text_blod(true)
            .time_text_blod(true)
            .show_time(true)
            .endl(true)
            .build(),
    );
    output(
        OutputBuilder::new_from(Output::default())
            .text("test_output_builder")
            .text_color(ColorType::Color256(0xffffff))
            .text_bg_color(ColorType::Color256(0xffffff))
            .split_bg_color(ColorType::Color256(0xffffff))
            .time_text_color(ColorType::Rgb(255, 200, 255))
            .text_blod(true)
            .time_text_blod(true)
            .show_time(true)
            .endl(true)
            .build(),
    );
}

#[test]
fn test_output_builder() {
    OutputBuilder::new()
        .text("test_output_builder_output")
        .text_bg_color(ColorType::Color256(0xffffff))
        .text_color(ColorType::Color256(0xffffff))
        .time_text_color(ColorType::Rgb(255, 200, 255))
        .text_blod(true)
        .time_text_blod(true)
        .show_time(true)
        .endl(true)
        .build()
        .output();
}
