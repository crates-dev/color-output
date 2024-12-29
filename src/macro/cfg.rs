use crate::*;

#[test]
fn test_proc_macro_output_struct() {
    output_macro!(Output {
        text: "test_proc_macro",
        color: ColorType::default(),
        bg_color: ColorType::Use(Color::Yellow),
        endl: true,
        ..Default::default()
    });
}

#[test]
fn test_proc_mcacro_output_builder() {
    output_macro!(OutputBuilder::new()
        .text("test_output_builder")
        .color(ColorType::Use(Color::Cyan))
        .blod(true)
        .endl(true)
        .build());
}

#[test]
fn test_proc_macro_multiple() {
    output_macro!(
        Output {
            text: "test_proc_macro",
            color: ColorType::default(),
            bg_color: ColorType::Use(Color::Yellow),
            endl: true,
            ..Default::default()
        },
        OutputBuilder::new()
            .text("test_output_builder1")
            .color(ColorType::Color256(0xffffff))
            .blod(true)
            .endl(true)
            .build(),
        OutputBuilder::new()
            .text("test_output_builder2")
            .color(ColorType::Color256(0xffffff))
            .blod(true)
            .endl(true)
            .build()
    );
}
