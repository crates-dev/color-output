#[test]
fn test_output_builder_new_from() {
    use crate::*;
    output(
        OutputBuilder::new()
            .text("test_output_builder")
            .color(ColorType::Color256(0xffffff))
            .bg_color(ColorType::Color256(0xffffff))
            .bold(true)
            .endl(true)
            .build(),
    );
    output(
        OutputBuilder::new_from(Output::default())
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
    use crate::*;
    OutputBuilder::new()
        .text("test_output_builder_output")
        .bg_color(ColorType::Color256(0xffffff))
        .color(ColorType::Color256(0xffffff))
        .bold(true)
        .endl(true)
        .build()
        .output();
}
