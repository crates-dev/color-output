use crate::*;
#[test]
fn test_output_builder_new_from() {
    output(
        OutputBuilder::new()
            .set_text("test_output_builder")
            .set_text_color(ColorType::Color256(0xffffff))
            .set_text_bg_color(ColorType::Color256(0xffffff))
            .set_split_bg_color(ColorType::Color256(0xffffff))
            .set_time_text_color(ColorType::Rgb(255, 200, 255))
            .set_text_blod(true)
            .set_time_text_blod(true)
            .set_show_time(true)
            .set_endl(true)
            .build(),
    );
    output(
        OutputBuilder::new_from(Output::default())
            .set_text("test_output_builder")
            .set_text_color(ColorType::Color256(0xffffff))
            .set_text_bg_color(ColorType::Color256(0xffffff))
            .set_split_bg_color(ColorType::Color256(0xffffff))
            .set_time_text_color(ColorType::Rgb(255, 200, 255))
            .set_text_blod(true)
            .set_time_text_blod(true)
            .set_show_time(true)
            .set_endl(true)
            .build(),
    );
}

#[test]
fn test_output_builder() {
    OutputBuilder::new()
        .set_text("test_output_builder_output")
        .set_text_bg_color(ColorType::Color256(0xffffff))
        .set_text_color(ColorType::Color256(0xffffff))
        .set_time_text_color(ColorType::Rgb(255, 200, 255))
        .set_text_blod(true)
        .set_time_text_blod(true)
        .set_show_time(true)
        .set_endl(true)
        .build()
        .output();
}
