use crate::*;

#[test]
fn test_color() {
    use color::r#const::DEFAULT;
    let color: Color = Color::Default;
    let color_str: &String = &color.to_string();
    assert_eq!(color_str, DEFAULT);
}

#[test]
fn test_color_get_str() {
    let color_str: &str = &Color::Default.get_str(DisplayType::Text);
    let res_color_str: &String = &Color::Default.to_string();
    assert_eq!(color_str, res_color_str);
}

#[test]
fn test_bg_color() {
    use color::r#const::DEFAULT;
    let bg_color: Color = Color::Default;
    let bg_color_str: &String = &bg_color.to_string();
    assert_eq!(bg_color_str, DEFAULT);
}

#[test]
fn test_bg_color_get_str() {
    let bg_color_str: &str = &Color::Default.get_str(DisplayType::Background);
    let ans_bg_color_str: &String = &Color::Default.to_string();
    assert_eq!(bg_color_str, ans_bg_color_str);
}

#[test]
fn test_color256_fg_color() {
    use color::r#fn::color256_fg_color;
    let color_str: String = color256_fg_color(0x3f3f3f);
    let ans_color_str: String = format!("38;5;{}", 237);
    assert_eq!(color_str, ans_color_str);
}

#[test]
fn test_color256_bg_color() {
    use color::r#fn::color256_bg_color;
    let color_str: String = color256_bg_color(0x000000);
    let ans_color_str: String = format!("48;5;{}", 16);
    assert_eq!(color_str, ans_color_str);
}

#[test]
fn test_rgb_fg_color() {
    use color::r#fn::rgb_fg_color;
    let color_str: String = rgb_fg_color(255, 255, 255);
    let ans_color_str: String = format!("38;2;{};{};{}", 255, 255, 255);
    assert_eq!(color_str, ans_color_str);
}

#[test]
fn test_rgb_bg_color() {
    use color::r#fn::rgb_bg_color;
    let color_str: String = rgb_bg_color(0, 0, 0);
    let ans_color_str: String = format!("48;2;{};{};{}", 0, 0, 0);
    assert_eq!(color_str, ans_color_str);
}

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

#[test]
fn test_output_builder_new_from() {
    output(
        ColorOutputBuilder::new()
            .text("test_output_builder")
            .color(ColorType::Color256(0xffffff))
            .bg_color(ColorType::Color256(0xffffff))
            .bold(true)
            .endl(true)
            .build(),
    );
    output(
        ColorOutputBuilder::new_from(ColorOutput::default())
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
    ColorOutputBuilder::new()
        .text("test_output_builder_output")
        .bg_color(ColorType::Color256(0xffffff))
        .color(ColorType::Color256(0xffffff))
        .bold(true)
        .endl(true)
        .build()
        .output();
}

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
