use crate::*;

#[test]
fn test_output_struct_function() {
    output(ColorOutput {
        text: "test_output_struct",
        color: ColorType::Use(Color::Default),
        bg_color: ColorType::Color256(0x000000),
        endl: true,
        ..Default::default()
    });
}

#[test]
fn test_output_struct_output_method() {
    ColorOutput {
        text: "test_output_struct_output",
        color: ColorType::Use(Color::Default),
        bg_color: ColorType::Use(Color::Blue),
        endl: true,
        ..Default::default()
    }
    .output();
    ColorOutput {
        text: "test_output_struct_output",
        color: ColorType::Use(Color::White),
        bg_color: ColorType::Use(Color::Blue),
        endl: true,
        ..Default::default()
    }
    .output();
}
