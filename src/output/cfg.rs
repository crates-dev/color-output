#[test]
fn test_output_struct_function() {
    use crate::*;
    output(Output {
        text: "test_output_struct",
        color: ColorType::Use(Color::Default),
        bg_color: ColorType::Color256(0x000000),
        endl: true,
        ..Default::default()
    });
}

#[test]
fn test_output_struct_output_method() {
    use crate::*;
    Output {
        text: "test_output_struct_output",
        color: ColorType::Use(Color::Default),
        bg_color: ColorType::Use(Color::Blue),
        endl: true,
        ..Default::default()
    }
    .output();
}
