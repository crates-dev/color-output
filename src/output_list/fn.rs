use crate::*;

/// Executes a sequence of output operations.
///
/// # Arguments
///
/// - `Vec<Output>` - Collection of output configurations to execute
///
/// # Returns
///
/// - `()` - No return value
pub fn output_list(output_list: &Vec<Output>) {
    let mut task_list: Task<'_> = Task::default();
    for output in output_list {
        let text: &str = output.text;
        let color: ColorType = output.color;
        let bg_color: ColorType = output.bg_color;
        let blod: bool = output.blod;
        let endl: bool = output.endl;
        task_list.add(Text {
            text,
            color,
            bg_color,
            blod,
            endl,
        });
    }
    task_list.run_all();
}
