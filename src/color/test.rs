#[test]
fn test_color() {
    use crate::*;
    use color::r#const::DEFAULT;
    let color: Color = Color::Default;
    let color_str: &String = &color.to_string();
    assert_eq!(color_str, DEFAULT);
}

#[test]
fn test_color_get_str() {
    use crate::*;
    let color_str: &str = &Color::Default.get_str(DisplayType::Text);
    let res_color_str: &String = &Color::Default.to_string();
    assert_eq!(color_str, res_color_str);
}

#[test]
fn test_bg_color() {
    use crate::*;
    use color::r#const::DEFAULT;
    let bg_color: Color = Color::Default;
    let bg_color_str: &String = &bg_color.to_string();
    assert_eq!(bg_color_str, DEFAULT);
}

#[test]
fn test_bg_color_get_str() {
    use crate::*;
    let bg_color_str: &str = &Color::Default.get_str(DisplayType::Background);
    let ans_bg_color_str: &String = &Color::Default.to_string();
    assert_eq!(bg_color_str, ans_bg_color_str);
}

#[test]
fn test_color256_fg_color() {
    use crate::*;
    use color::r#fn::color256_fg_color;
    let color_str: String = color256_fg_color(0x3f3f3f);
    let ans_color_str: String = format!("38;5;{}", 237);
    assert_eq!(color_str, ans_color_str);
}

#[test]
fn test_color256_bg_color() {
    use crate::*;
    use color::r#fn::color256_bg_color;
    let color_str: String = color256_bg_color(0x000000);
    let ans_color_str: String = format!("48;5;{}", 16);
    assert_eq!(color_str, ans_color_str);
}

#[test]
fn test_rgb_fg_color() {
    use crate::*;
    use color::r#fn::rgb_fg_color;
    let color_str: String = rgb_fg_color(255, 255, 255);
    let ans_color_str: String = format!("38;2;{};{};{}", 255, 255, 255);
    assert_eq!(color_str, ans_color_str);
}

#[test]
fn test_rgb_bg_color() {
    use crate::*;
    use color::r#fn::rgb_bg_color;
    let color_str: String = rgb_bg_color(0, 0, 0);
    let ans_color_str: String = format!("48;2;{};{};{}", 0, 0, 0);
    assert_eq!(color_str, ans_color_str);
}
