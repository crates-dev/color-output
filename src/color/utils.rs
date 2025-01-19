/// Generates an ANSI escape sequence for foreground colors in 256 colors.
///
/// # Parameters
/// - `code`: The color code, which ranges from 0 to 255.
///
/// # Returns
/// - `String`: Returns a string containing the ANSI escape sequence for the foreground color.
#[inline]
pub fn color256_fg_color(code: u32) -> String {
    format!("\x1b[38;5;{}m", code)
}

/// Generates an ANSI escape sequence for background colors in 256 colors.
///
/// # Parameters
/// - `code`: The color code, which ranges from 0 to 255.
///
/// # Returns
/// - `String`: Returns a string containing the ANSI escape sequence for the background color.
#[inline]
pub fn color256_bg_color(code: u32) -> String {
    // Check if the color range is between 0x000000 and 0xFFFFFF
    if code > 0xFFFFFF {
        // If it exceeds the range, return an empty string
        return String::new();
    }
    // Extract RGB components
    let r: u32 = (code >> 16) & 0xFF;
    let g: u32 = (code >> 8) & 0xFF;
    let b: u32 = code & 0xFF;
    // Map RGB components to 256 color index
    let r_index: u32 = (r * 6 / 256) as u32;
    let g_index: u32 = (g * 6 / 256) as u32;
    let b_index: u32 = (b * 6 / 256) as u32;
    // Calculate the final 256 color index
    let color_index: u32 = 16 + 36 * r_index + 6 * g_index + b_index;
    format!("\x1b[48;5;{}m", color_index)
}

/// Generates an ANSI escape sequence for true color foreground colors.
///
/// # Parameters
/// - `r`: The red component, which ranges from 0 to 255.
/// - `g`: The green component, which ranges from 0 to 255.
/// - `b`: The blue component, which ranges from 0 to 255.
///
/// # Returns
/// - `String`: Returns a string containing the ANSI escape sequence for the true color foreground color.
#[inline]
pub fn rgb_fg_color(r: u8, g: u8, b: u8) -> String {
    format!("\x1b[38;2;{};{};{}m", r, g, b)
}

/// Generates an ANSI escape sequence for true color background colors.
///
/// # Parameters
/// - `r`: The red component, which ranges from 0 to 255.
/// - `g`: The green component, which ranges from 0 to 255.
/// - `b`: The blue component, which ranges from 0 to 255.
///
/// # Returns
/// - `String`: Returns a string containing the ANSI escape sequence for the true color background color.
#[inline]
pub fn rgb_bg_color(r: u8, g: u8, b: u8) -> String {
    format!("\x1b[48;2;{};{};{}m", r, g, b)
}
