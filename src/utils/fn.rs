use crate::*;

pub fn __println_text(color: ColorType, bg_color: ColorType, text: &str) {
    let binding: String = format!("[{}]", time());
    let mut time_output_builder: OutputBuilder<'_> = OutputBuilder::new();
    let mut text_output_builder: OutputBuilder<'_> = OutputBuilder::new();
    let time_output: Output<'_> = time_output_builder
        .text(&binding)
        .blod(true)
        .color(color)
        .bg_color(bg_color)
        .build();
    let mut lines = text.lines().peekable();
    while let Some(line) = lines.next() {
        let mut output_list_builder = OutputListBuilder::new();
        output_list_builder.add(time_output);
        let text_output: Output<'_> = text_output_builder.text(line).blod(true).endl(true).build();
        output_list_builder.add(text_output);
        output_list_builder.run();
    }
}
