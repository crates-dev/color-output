#[test]
fn test_proc_macro_output_struct() {
    use crate::*;
    output_macro!(Output {
        text: "test_proc_macro",
        color: ColorType::default(),
        bg_color: ColorType::Use(Color::Yellow),
        endl: true,
        ..Default::default()
    });
}

#[test]
fn test_proc_macro_output_builder() {
    use crate::*;
    output_macro!(
        OutputBuilder::new()
            .text("test_output_builder")
            .color(ColorType::Use(Color::Cyan))
            .blod(true)
            .endl(true)
            .build()
    );
}

#[test]
fn test_proc_macro_multiple() {
    use crate::*;
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

#[test]
fn test_print_type() {
    use crate::*;
    let msg: &str = "1\n2\n3\r\n4";
    println_success!(msg);
    println!("==========");
    println_warning!(msg);
    println!("==========");
    println_error!(msg);
    println!("==========");
    let msg: &str = "1234";
    println_success!(msg, msg);
    println!("==========");
    println_warning!(msg, msg);
    println!("==========");
    println_error!(msg, msg);
    println!("==========");
}
