use crate::*;

#[test]
fn test_output_builder_new_from() {
    output(
        ColorOutputBuilder::new()
            .text("test_output_builder")
            .color(ColorType::Color256(0xffffff))
            .bg_color(ColorType::Color256(0xffffff))
            .bold(true)
            .endl(true)
            .build(),
    );
    output(
        ColorOutputBuilder::new_from(ColorOutput::default())
            .text("test_output_builder")
            .color(ColorType::Color256(0xffffff))
            .bg_color(ColorType::Color256(0xffffff))
            .bold(true)
            .endl(true)
            .build(),
    );
}

#[test]
fn test_output_builder() {
    ColorOutputBuilder::new()
        .text("test_output_builder_output")
        .bg_color(ColorType::Color256(0xffffff))
        .color(ColorType::Color256(0xffffff))
        .bold(true)
        .endl(true)
        .build()
        .output();
}
