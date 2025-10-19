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
/// - Format string and arguments (same as `format!` macro)
///
/// Used by the success/warning/error printing macros to avoid code duplication.
#[macro_export]
macro_rules! __print_message_common {
    ($color:expr, $bg_color:expr, $($arg:tt)*) => {{
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
        let text: String = format!($($arg)*);
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
///
/// Supports format string syntax like `format!` macro:
/// - `println_success!("Hello")` - Simple string
/// - `println_success!("Hello {}", name)` - Positional arguments
/// - `println_success!("Hello {name}")` - Named arguments (Rust 1.58+)
#[macro_export]
macro_rules! println_success {
    ($($arg:tt)*) => {
        $crate::__print_message_common!(ColorType::Use(Color::White), ColorType::Use(Color::Green), $($arg)*);
    };
}

/// Prints a warning message with yellow background and white text.
///
/// Supports format string syntax like `format!` macro:
/// - `println_warning!("Warning")` - Simple string
/// - `println_warning!("Warning: {}", error)` - Positional arguments
/// - `println_warning!("Warning: {error}")` - Named arguments (Rust 1.58+)
#[macro_export]
macro_rules! println_warning {
    ($($arg:tt)*) => {
        $crate::__print_message_common!(ColorType::Use(Color::White), ColorType::Use(Color::Yellow), $($arg)*);
    };
}

/// Prints an error message with red background and white text.
///
/// Supports format string syntax like `format!` macro:
/// - `println_error!("Error")` - Simple string
/// - `println_error!("Error: {}", message)` - Positional arguments
/// - `println_error!("Error: {message}")` - Named arguments (Rust 1.58+)
#[macro_export]
macro_rules! println_error {
    ($($arg:tt)*) => {
        $crate::__print_message_common!(ColorType::Use(Color::White), ColorType::Use(Color::Red), $($arg)*);
    };
}
