use crate::*;

/// Executes the output operation with the given formatting.
///
/// # Arguments
///
/// - `Output` - The output configuration
pub fn output(output: Output) {
    let text: &str = output.text;
    let color: ColorType = output.color;
    let bg_color: ColorType = output.bg_color;
    let bold: bool = output.bold;
    let endl: bool = output.endl;
    let mut task_list: Task<'_> = Task::default();
    task_list.add(Text {
        text,
        color,
        bg_color,
        bold,
        endl,
    });
    task_list.run_all();
}
