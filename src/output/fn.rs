use crate::*;

/// Executes the output operation with the given formatting.
///
/// [Official Documentation](https://docs.ltpp.vip/COLOR-OUTPUT/)
///
/// # Arguments
///
/// - `Output` - The output configuration
pub fn output(output: Output) {
    let text: &str = output.text;
    let color: ColorType = output.color.clone();
    let bg_color: ColorType = output.bg_color.clone();
    let blod: bool = output.blod.clone();
    let endl: bool = output.endl;
    let mut task_list: Task<'_> = Task::default();
    task_list.add(Text {
        text,
        color,
        bg_color,
        blod,
        endl,
    });
    task_list.run_all();
}
