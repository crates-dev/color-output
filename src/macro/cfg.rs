use crate::*;

#[test]
fn test_proc_macro_output_struct() {
    output_macro!(Output {
        text: "test_proc_macro",
        text_color: ColorType::default(),
        text_bg_color: ColorType::Use(Color::Yellow),
        endl: true,
        ..Default::default()
    });
}

#[test]
fn test_proc_mcacro_output_builder() {
    output_macro!(OutputBuilder::new()
        .text("test_output_builder")
        .text_color(ColorType::Use(Color::Cyan))
        .text_blod(true)
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
            endl: true,
            ..Default::default()
        },
        OutputBuilder::new()
            .text("test_output_builder1")
            .text_color(ColorType::Color256(0xffffff))
            .text_blod(true)
            .endl(true)
            .build(),
        OutputBuilder::new()
            .text("test_output_builder2")
            .text_color(ColorType::Color256(0xffffff))
            .text_blod(true)
            .endl(true)
            .build()
    );
}
