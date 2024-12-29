use crate::*;
#[test]
fn test_output_builder_new_from() {
    output(
        OutputBuilder::new()
            .text("test_output_builder")
            .color(ColorType::Color256(0xffffff))
            .bg_color(ColorType::Color256(0xffffff))
            .blod(true)
            .endl(true)
            .build(),
    );
    output(
        OutputBuilder::new_from(Output::default())
            .text("test_output_builder")
            .color(ColorType::Color256(0xffffff))
            .bg_color(ColorType::Color256(0xffffff))
            .blod(true)
            .endl(true)
            .build(),
    );
}

#[test]
fn test_output_builder() {
    OutputBuilder::new()
        .text("test_output_builder_output")
        .bg_color(ColorType::Color256(0xffffff))
        .color(ColorType::Color256(0xffffff))
        .blod(true)
        .endl(true)
        .build()
        .output();
}
