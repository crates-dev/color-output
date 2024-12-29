use crate::*;
use output_list_builder::r#type::OutputListBuilder;

#[test]
fn test_new_from_output_list_builder() {
    OutputListBuilder::new_from(vec![Output::default()])
        .add(
            OutputBuilder::new()
                .text("text")
                .bg_color(ColorType::Use(Color::Blue))
                .endl(false)
                .build(),
        )
        .add(Output {
            text: "test_new_from_output_list_builder_1",
            color: ColorType::Use(Color::Default),
            bg_color: ColorType::Color256(0x3f3f3f),
            endl: false,
            ..Default::default()
        })
        .add(Output {
            text: "test_new_from_output_list_builder_2",
            color: ColorType::Use(Color::Default),
            bg_color: ColorType::Use(Color::Cyan),
            endl: true,
            ..Default::default()
        })
        .run();
}
