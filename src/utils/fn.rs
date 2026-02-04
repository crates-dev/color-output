use crate::*;

pub fn __println_text(color: ColorType, bg_color: ColorType, text: &str) {
    let binding: String = format!("[{}]", time());
    let mut time_output_builder: ColorOutputBuilder<'_> = ColorOutputBuilder::new();
    let mut text_output_builder: ColorOutputBuilder<'_> = ColorOutputBuilder::new();
    let time_output: ColorOutput<'_> = time_output_builder
        .text(&binding)
        .bold(true)
        .color(color)
        .bg_color(bg_color)
        .build();
    let lines = text.lines().peekable();
    for line in lines {
        let mut output_list_builder = ColorOutputListBuilder::new();
        output_list_builder.add(time_output);
        let text_output: ColorOutput<'_> =
            text_output_builder.text(line).bold(true).endl(true).build();
        output_list_builder.add(text_output);
        output_list_builder.run();
    }
}
