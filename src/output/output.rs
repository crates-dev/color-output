use task::r#type::Task;

use crate::text::r#type::Text;
use crate::time::time::get_now_time_format;
use crate::*;
use std::borrow::Cow;

/// Output
///
/// [Official Documentation](https://docs.ltpp.vip/LTPP-RUST-OUTPUT/),
///
/// # Parameters
/// - `Output`: The output struct
///
/// # Code Example
///
/// ## Using the Struct
///
/// ### Using the output Function
///
/// ```rust
/// use ltpp_output::*;
/// ```
///
/// ## Using the Constructor
///
/// ### Using the output Function
///
/// ```rust
/// use ltpp_output::*;
/// ```
pub fn output(output: Output) {
    // Text
    let text: &str = output.text;
    let text_color: ColorType = output.text_color.clone();
    let text_bg_color: ColorType = output.text_bg_color.clone();
    let text_blod: bool = output.text_blod.clone();
    // Time
    let show_time: bool = output.show_time.clone();
    let time_color: ColorType = output.time_text_color.clone();
    let time_bg_color: ColorType = output.time_bg_color.clone();
    let time_text_blod: bool = output.time_text_blod.clone();
    // Separator
    let split: &str = output.split.clone();
    let split_color: ColorType = output.split_color.clone();
    let split_bg_color: ColorType = output.split_bg_color.clone();
    let split_text_blod: bool = output.time_text_blod.clone();
    // endl
    let endl: bool = output.endl;
    let mut output: String = String::new();

    let mut task_list: Task<'_> = Task::new();

    // Add time
    let time_str: &String = if show_time {
        &format!("[{}]", get_now_time_format())
    } else {
        &String::new()
    };
    task_list.add(Text {
        text: time_str,
        text_color: time_color,
        text_bg_color: time_bg_color,
        blod: time_text_blod,
        endl: false,
    });

    // Add separator
    task_list.add(Text {
        text: split,
        text_color: split_color,
        text_bg_color: split_bg_color,
        blod: split_text_blod,
        endl: false,
    });

    // Add text
    task_list.add(Text {
        text,
        text_color,
        text_bg_color,
        blod: text_blod,
        endl,
    });

    // run
    task_list.run_all();
}
