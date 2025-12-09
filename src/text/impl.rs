use super::r#struct::*;
use crate::color::{blod::*, r#const::*};
use crate::*;
use std::borrow::Cow;

/// Default implementation for Text with empty content and default styling.
impl<'a> Default for Text<'a> {
    #[inline(always)]
    fn default() -> Self {
        Text {
            text: "",
            color: ColorType::default(),
            bg_color: ColorType::default(),
            blod: false,
            endl: false,
        }
    }
}

impl<'a> Text<'a> {
    /// Creates a new Text instance from existing configuration.
    ///
    /// # Arguments
    ///
    /// - `Text` - Source text configuration to clone
    ///
    /// # Returns
    ///
    /// - `Text` - New instance with cloned configuration
    #[inline(always)]
    pub(crate) fn new_from(text: &Text<'a>) -> Self {
        Self { ..text.clone() }
    }

    /// Gets the display string as a `Cow` (clone on write).
    ///
    /// This method generates a formatted string that represents the text with
    /// the appropriate color and background color. If the text is bold, it applies
    /// bold formatting to the text color.
    ///
    /// # Returns
    /// - `Cow<'a, str>`: An owned copy of the formatted string.
    pub(crate) fn get_display_str_cow(&self) -> Cow<'a, str> {
        let text: &str = self.text;
        let blod: bool = self.blod;

        let adjusted_color: ColorType = if matches!(self.color, ColorType::Use(Color::Default)) {
            ColorContrast::ensure_sufficient_contrast(&self.color, &self.bg_color)
        } else {
            self.color.clone()
        };
        let color: &String = &adjusted_color.to_string();
        let bg_color: &String = &self.bg_color.get_str(DisplayType::Background);
        let mut colored_text: String = if blod {
            format!("{bg_color}{color}{BLOD}{text}{UNBLOD}{RESET}")
        } else {
            format!("{bg_color}{color}{text}{RESET}")
        };
        if self.endl {
            colored_text.push('\n');
        }
        Cow::Owned(colored_text)
    }
}
