use crate::*;

impl ColorContrast {
    /// Calculates the relative luminance of a color.
    ///
    /// # Arguments
    ///
    /// - `u8` - Red component (0-255)
    /// - `u8` - Green component (0-255)
    /// - `u8` - Blue component (0-255)
    ///
    /// # Returns
    ///
    /// - `f64` - Relative luminance value (0.0-1.0)
    pub fn calculate_luminance(r: u8, g: u8, b: u8) -> f64 {
        let r_norm: f64 = r as f64 / 255.0;
        let g_norm: f64 = g as f64 / 255.0;
        let b_norm: f64 = b as f64 / 255.0;
        let r_linear: f64 = if r_norm <= 0.03928 {
            r_norm / 12.92
        } else {
            ((r_norm + 0.055) / 1.055).powf(2.4)
        };
        let g_linear: f64 = if g_norm <= 0.03928 {
            g_norm / 12.92
        } else {
            ((g_norm + 0.055) / 1.055).powf(2.4)
        };
        let b_linear: f64 = if b_norm <= 0.03928 {
            b_norm / 12.92
        } else {
            ((b_norm + 0.055) / 1.055).powf(2.4)
        };
        0.2126 * r_linear + 0.7152 * g_linear + 0.0722 * b_linear
    }

    /// Calculates the contrast ratio between two colors.
    ///
    /// # Arguments
    ///
    /// - `(u8, u8, u8)` - First color RGB values
    /// - `(u8, u8, u8)` - Second color RGB values
    ///
    /// # Returns
    ///
    /// - `f64` - Contrast ratio (1.0-21.0)
    pub fn calculate_contrast_ratio(color1: (u8, u8, u8), color2: (u8, u8, u8)) -> f64 {
        let lum1: f64 = Self::calculate_luminance(color1.0, color1.1, color1.2);
        let lum2: f64 = Self::calculate_luminance(color2.0, color2.1, color2.2);
        let lighter: f64 = lum1.max(lum2);
        let darker: f64 = lum1.min(lum2);
        (lighter + 0.05) / (darker + 0.05)
    }

    /// Extracts RGB values from ColorType.
    ///
    /// # Arguments
    ///
    /// - `&ColorType` - Color to extract RGB from
    ///
    /// # Returns
    ///
    /// - `(u8, u8, u8)` - RGB values
    pub fn extract_rgb_from_color_type(color: &ColorType) -> (u8, u8, u8) {
        match color {
            ColorType::Rgb(r, g, b) => (*r, *g, *b),
            ColorType::Color256(hex) => {
                let r: u8 = ((hex >> 16) & 0xFF) as u8;
                let g: u8 = ((hex >> 8) & 0xFF) as u8;
                let b: u8 = (hex & 0xFF) as u8;
                (r, g, b)
            }
            ColorType::Use(color) => {
                use super::r#type::Color;
                match color {
                    Color::Default => (128, 128, 128),
                    Color::Black => (0, 0, 0),
                    Color::Red => (255, 0, 0),
                    Color::Green => (0, 255, 0),
                    Color::Yellow => (255, 255, 0),
                    Color::Blue => (0, 0, 255),
                    Color::Magenta => (255, 0, 255),
                    Color::Cyan => (0, 255, 255),
                    Color::White => (255, 255, 255),
                }
            }
        }
    }

    /// Checks if two colors have sufficient contrast for readability.
    ///
    /// # Arguments
    ///
    /// - `&ColorType` - Text color
    /// - `&ColorType` - Background color
    ///
    /// # Returns
    ///
    /// - `bool` - Whether contrast is sufficient (ratio >= 4.5)
    pub fn has_sufficient_contrast(text_color: &ColorType, bg_color: &ColorType) -> bool {
        let text_rgb: (u8, u8, u8) = Self::extract_rgb_from_color_type(text_color);
        let bg_rgb: (u8, u8, u8) = Self::extract_rgb_from_color_type(bg_color);
        let ratio: f64 = Self::calculate_contrast_ratio(text_rgb, bg_rgb);
        ratio >= 4.5
    }

    /// Automatically adjusts text color to ensure sufficient contrast with background.
    ///
    /// # Arguments
    ///
    /// - `&ColorType` - Original text color
    /// - `&ColorType` - Background color
    ///
    /// # Returns
    ///
    /// - `ColorType` - Adjusted text color with sufficient contrast
    pub fn ensure_sufficient_contrast(text_color: &ColorType, bg_color: &ColorType) -> ColorType {
        if Self::has_sufficient_contrast(text_color, bg_color) {
            return text_color.clone();
        }
        let text_rgb: (u8, u8, u8) = Self::extract_rgb_from_color_type(text_color);
        let bg_rgb: (u8, u8, u8) = Self::extract_rgb_from_color_type(bg_color);
        let bg_luminance: f64 = Self::calculate_luminance(bg_rgb.0, bg_rgb.1, bg_rgb.2);
        if bg_luminance > 0.5 {
            Self::darken_color_for_contrast(text_rgb, bg_rgb)
        } else {
            Self::lighten_color_for_contrast(text_rgb, bg_rgb)
        }
    }

    /// Darkens a color while preserving its hue for better contrast against light backgrounds.
    ///
    /// # Arguments
    ///
    /// - `(u8, u8, u8)` - Original text color RGB
    /// - `(u8, u8, u8)` - Background color RGB
    ///
    /// # Returns
    ///
    /// - `ColorType` - Darkened color with sufficient contrast
    fn darken_color_for_contrast(text_rgb: (u8, u8, u8), bg_rgb: (u8, u8, u8)) -> ColorType {
        let (r, g, b): (u8, u8, u8) = text_rgb;
        let max_component: u8 = r.max(g).max(b);
        if max_component == 0 {
            return ColorType::Rgb(0, 0, 0);
        }
        let scale_factor: f64 = 0.3;
        let new_r: u8 = ((r as f64 * scale_factor) as u8).min(80);
        let new_g: u8 = ((g as f64 * scale_factor) as u8).min(80);
        let new_b: u8 = ((b as f64 * scale_factor) as u8).min(80);
        let result_color: ColorType = ColorType::Rgb(new_r, new_g, new_b);
        if Self::calculate_contrast_ratio((new_r, new_g, new_b), bg_rgb) >= 4.5 {
            result_color
        } else {
            ColorType::Rgb(0, 0, 0)
        }
    }

    /// Lightens a color while preserving its hue for better contrast against dark backgrounds.
    ///
    /// # Arguments
    ///
    /// - `(u8, u8, u8)` - Original text color RGB
    /// - `(u8, u8, u8)` - Background color RGB
    ///
    /// # Returns
    ///
    /// - `ColorType` - Lightened color with sufficient contrast
    fn lighten_color_for_contrast(text_rgb: (u8, u8, u8), bg_rgb: (u8, u8, u8)) -> ColorType {
        let (r, g, b): (u8, u8, u8) = text_rgb;
        let scale_factor: f64 = 2.5;
        let new_r: u8 = ((r as f64 * scale_factor) as u8).min(255).max(200);
        let new_g: u8 = ((g as f64 * scale_factor) as u8).min(255).max(200);
        let new_b: u8 = ((b as f64 * scale_factor) as u8).min(255).max(200);
        let result_color: ColorType = ColorType::Rgb(new_r, new_g, new_b);
        if Self::calculate_contrast_ratio((new_r, new_g, new_b), bg_rgb) >= 4.5 {
            result_color
        } else {
            ColorType::Rgb(255, 255, 255)
        }
    }
}
