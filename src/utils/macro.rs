/// Macro for outputting colored text to the terminal.
///
/// # Arguments
///
/// - `Output` or `OutputBuilder` - One or more output instances to execute
#[macro_export]
macro_rules! output_macro {
    ($($output:expr),*) => {
        $($output.output();)*
    };
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
        $crate::__println_text(ColorType::Use(Color::White), ColorType::Use(Color::Green), &format!($($arg)*));
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
        $crate::__println_text(ColorType::Use(Color::White), ColorType::Use(Color::Yellow), &format!($($arg)*));
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
        $crate::__println_text(ColorType::Use(Color::White), ColorType::Use(Color::Red), &format!($($arg)*));
    };
}
