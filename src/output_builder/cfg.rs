use crate::*;
#[test]
fn test_output_builder_new_from() {
    output(
        OutputBuilder::new()
            .text("test_output_builder")
            .text_color(ColorType::Color256(0xffffff))
            .text_bg_color(ColorType::Color256(0xffffff))
            .text_blod(true)
            .endl(true)
            .build(),
    );
    output(
        OutputBuilder::new_from(Output::default())
            .text("test_output_builder")
            .text_color(ColorType::Color256(0xffffff))
            .text_bg_color(ColorType::Color256(0xffffff))
            .text_blod(true)
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
        .text_blod(true)
        .endl(true)
        .build()
        .output();
}
