use crate::*;

#[test]
fn test_output_list_struct() {
    ColorOutputList(vec![
        ColorOutput {
            text: "test_output_list_struct_1",
            color: ColorType::Use(Color::Default),
            bg_color: ColorType::Color256(0x000000),
            endl: false,
            ..Default::default()
        },
        ColorOutput {
            text: "test_output_struct_output_2",
            color: ColorType::Use(Color::Default),
            bg_color: ColorType::Use(Color::Blue),
            endl: true,
            ..Default::default()
        },
    ])
    .output();
}
