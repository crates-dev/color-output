use crate::*;

#[test]
fn test_new_output_list_builder() {
    ColorOutputListBuilder::new()
        .add(
            ColorOutputBuilder::new()
                .text("text")
                .bg_color(ColorType::Use(Color::Blue))
                .endl(false)
                .build(),
        )
        .add(ColorOutput {
            text: "test_new_output_list_builder_1",
            color: ColorType::Use(Color::Default),
            bg_color: ColorType::Color256(0x3f3f3f),
            endl: false,
            ..Default::default()
        })
        .add(ColorOutput {
            text: "test_new_output_list_builder_2",
            color: ColorType::Use(Color::Default),
            bg_color: ColorType::Use(Color::Cyan),
            endl: true,
            ..Default::default()
        })
        .run();
}

#[test]
fn test_new_from_output_list_builder() {
    ColorOutputListBuilder::new_from(vec![ColorOutput::default()])
        .add(
            ColorOutputBuilder::new()
                .text("text")
                .bg_color(ColorType::Use(Color::Blue))
                .endl(false)
                .build(),
        )
        .add(ColorOutput {
            text: "test_new_from_output_list_builder_1",
            color: ColorType::Use(Color::Default),
            bg_color: ColorType::Color256(0x3f3f3f),
            endl: false,
            ..Default::default()
        })
        .add(ColorOutput {
            text: "test_new_from_output_list_builder_2",
            color: ColorType::Use(Color::Default),
            bg_color: ColorType::Use(Color::Cyan),
            endl: true,
            ..Default::default()
        })
        .run();
}
