/// Macro for outputting colored text to the terminal.
///
/// # Arguments
///
/// - `Output` or `OutputBuilder` - One or more output instances to execute
#[macro_export]
macro_rules! output_macro {
    ($($output:expr),*) => {
        $(
            $output.output();
        )*
    };
}

/// Internal macro for handling common message printing logic.
///
/// # Arguments
///
/// - `ColorType` - Text color
/// - `ColorType` - Background color
/// - `&str` - One or more message strings to print
///
/// Used by the success/warning/error printing macros to avoid code duplication.
#[macro_export]
macro_rules! __print_message_common {
    ($color:expr, $bg_color:expr, $($data:expr),*) => {{
        use $crate::*;
        let binding: String = format!("[{}]", time());
        let mut time_output_builder: OutputBuilder<'_> = OutputBuilder::new();
        let time_output: Output<'_> = time_output_builder
            .text(&binding)
            .blod(true)
            .color($color)
            .bg_color($bg_color)
            .build();
        let mut text_output_builder: OutputBuilder<'_> = OutputBuilder::new();
        let mut text: String = String::new();
        $(
            text.push_str(&$data.to_string());
        )*
        let mut lines_iter = text.lines().peekable();
        while let Some(line) = lines_iter.next() {
            let mut output_list_builder = OutputListBuilder::new();
            output_list_builder.add(time_output.clone());
            let text_output: Output<'_> = text_output_builder
                .text(&line)
                .blod(true)
                .endl(true)
                .build();
            output_list_builder.add(text_output);
            output_list_builder.run();
        }
    }};
}

/// Prints a success message with green background and white text.
#[macro_export]
macro_rules! println_success {
    ($($data:expr),*) => {
        $crate::__print_message_common!(ColorType::Use(Color::White), ColorType::Use(Color::Green), $($data),*);
    };
}

/// Prints a warning message with yellow background and white text.
#[macro_export]
macro_rules! println_warning {
    ($($data:expr),*) => {
        $crate::__print_message_common!(ColorType::Use(Color::White), ColorType::Use(Color::Yellow), $($data),*);
    };
}

/// Prints an error message with red background and white text.
#[macro_export]
macro_rules! println_error {
    ($($data:expr),*) => {
        $crate::__print_message_common!(ColorType::Use(Color::White), ColorType::Use(Color::Red), $($data),*);
    };
}
