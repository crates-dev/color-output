use crate::{
    print_error, print_success, print_warning, println_error, println_success, println_warning,
};

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
fn test_proc_mcacro_output_builder() {
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
    print_success!("1");
    print_warning!("2");
    print_error!("3");
    println_success!("1");
    println_warning!("2");
    println_error!("3");
}
