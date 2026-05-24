use crate::*;

#[test]
fn test_proc_macro_output_struct() {
    output_macro!(ColorOutput {
        text: "test_proc_macro",
        color: ColorType::default(),
        bg_color: ColorType::Use(Color::Yellow),
        endl: true,
        ..Default::default()
    });
}

#[test]
fn test_proc_macro_output_builder() {
    output_macro!(
        ColorOutputBuilder::new()
            .text("test_output_builder")
            .color(ColorType::Use(Color::Cyan))
            .bold(true)
            .endl(true)
            .build()
    );
}

#[test]
fn test_proc_macro_multiple() {
    output_macro!(
        ColorOutput {
            text: "test_proc_macro",
            color: ColorType::default(),
            bg_color: ColorType::Use(Color::Yellow),
            endl: true,
            ..Default::default()
        },
        ColorOutputBuilder::new()
            .text("test_output_builder1")
            .color(ColorType::Color256(0xffffff))
            .bold(true)
            .endl(true)
            .build(),
        ColorOutputBuilder::new()
            .text("test_output_builder2")
            .color(ColorType::Color256(0xffffff))
            .bold(true)
            .endl(true)
            .build(),
        ColorOutput {
            text: "test_proc_macro",
            color: ColorType::default(),
            bg_color: ColorType::Use(Color::Yellow),
            endl: true,
            ..Default::default()
        }
    );
}

#[test]
fn test_print_type() {
    let msg: &str = "1\n2\n3\r\n4";
    println_success!("{msg}");
    println!("==========");
    println_warning!("{msg}");
    println!("==========");
    println_error!("{msg}");
    println!("==========");
    let msg: &str = "1234";
    println_success!("{msg}{msg}");
    println!("==========");
    println_warning!("{msg}{msg}");
    println!("==========");
    println_error!("{msg}{msg}");
    println!("==========");
}
