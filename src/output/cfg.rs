use crate::*;
#[test]
fn test_output_struct_function() {
    output(Output {
        text: "test_output_struct",
        text_color: ColorType::Use(Color::Default),
        text_bg_color: ColorType::Color256(0x000000),
        endl: true,
        ..Default::default()
    });
}

#[test]
fn test_output_struct_output_method() {
    Output {
        text: "test_output_struct_output",
        text_color: ColorType::Use(Color::Default),
        text_bg_color: ColorType::Use(Color::Blue),
        endl: true,
        ..Default::default()
    }
    .output();
}
