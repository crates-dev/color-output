use super::r#type::*;
use crate::color::{blod::*, color::*};
use crate::*;
use std::borrow::Cow;

impl<'a> Default for Text<'a> {
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
    /// Creates a text structure
    ///
    /// # Parameters
    /// - `Text`: The text structure
    ///
    /// # Returns
    /// - `Text`: The text structure
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
        let blod: bool = self.blod.clone();
        let color: &String = &self.color.to_string();
        let bg_color: &String = &self.bg_color.get_str(DisplayType::Background);
        let mut colored_text: String = if blod {
            format!("{}{}{}{}{}{}", bg_color, color, BLOD, text, UNBLOD, RESET)
        } else {
            format!("{}{}{}{}", bg_color, color, text, RESET)
        };
        if self.endl {
            colored_text.push_str("\n");
        }
        Cow::Owned(colored_text)
    }
}
