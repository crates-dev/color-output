/// Generates an ANSI escape sequence for foreground colors in 256 colors.
///
/// # Arguments
///
/// - `u32` - RGB color code in hexadecimal format (0x000000 to 0xFFFFFF)
///
/// # Returns
///
/// - `String` - ANSI escape sequence for the foreground color
pub fn color256_fg_color(code: u32) -> String {
    if code > 0xFFFFFF {
        return String::new();
    }
    let r: u32 = (code >> 16) & 0xFF;
    let g: u32 = (code >> 8) & 0xFF;
    let b: u32 = code & 0xFF;
    let color_index: u32 = rgb_to_256_color_index(r as u8, g as u8, b as u8);
    format!("\x1b[38;5;{}m", color_index)
}

/// Generates an ANSI escape sequence for background colors in 256 colors.
///
/// # Arguments
///
/// - `u32` - RGB color code in hexadecimal format (0x000000 to 0xFFFFFF)
///
/// # Returns
///
/// - `String` - ANSI escape sequence for the background color
pub fn color256_bg_color(code: u32) -> String {
    if code > 0xFFFFFF {
        return String::new();
    }
    let r: u32 = (code >> 16) & 0xFF;
    let g: u32 = (code >> 8) & 0xFF;
    let b: u32 = code & 0xFF;
    let color_index: u32 = rgb_to_256_color_index(r as u8, g as u8, b as u8);
    format!("\x1b[48;5;{}m", color_index)
}

/// Generates an ANSI escape sequence for true color foreground colors.
///
/// # Arguments
///
/// - `u8` - Red component (0-255)
/// - `u8` - Green component (0-255)
/// - `u8` - Blue component (0-255)
///
/// # Returns
///
/// - `String` - ANSI escape sequence for the true color foreground
#[inline]
pub fn rgb_fg_color(r: u8, g: u8, b: u8) -> String {
    format!("\x1b[38;2;{};{};{}m", r, g, b)
}

/// Generates an ANSI escape sequence for true color background colors.
///
/// # Arguments
///
/// - `u8` - Red component (0-255)
/// - `u8` - Green component (0-255)
/// - `u8` - Blue component (0-255)
///
/// # Returns
///
/// - `String` - ANSI escape sequence for the true color background
#[inline]
pub fn rgb_bg_color(r: u8, g: u8, b: u8) -> String {
    format!("\x1b[48;2;{};{};{}m", r, g, b)
}

/// Converts RGB values to 256-color palette index with high precision.
///
/// # Arguments
///
/// - `u8` - Red component (0-255)
/// - `u8` - Green component (0-255)
/// - `u8` - Blue component (0-255)
///
/// # Returns
///
/// - `u32` - 256-color palette index (16-231 for RGB colors, 0-15 for standard colors, 232-255 for grayscale)
fn rgb_to_256_color_index(r: u8, g: u8, b: u8) -> u32 {
    if r == g && g == b {
        if r < 8 {
            return 16;
        } else if r > 248 {
            return 231;
        } else {
            return 232 + ((r as u32 - 8) * 24 / 248);
        }
    }
    let r_index: u32 = if r < 48 {
        0
    } else if r < 115 {
        1
    } else {
        (r as u32 - 35) / 40
    };
    let g_index: u32 = if g < 48 {
        0
    } else if g < 115 {
        1
    } else {
        (g as u32 - 35) / 40
    };
    let b_index: u32 = if b < 48 {
        0
    } else if b < 115 {
        1
    } else {
        (b as u32 - 35) / 40
    };
    16 + 36 * r_index + 6 * g_index + b_index
}
