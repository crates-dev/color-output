use color::blod::{BLOD, UNBLOD};
use color::color::RESET;

use crate::color::color::DEFAULT;
use crate::DisplayType;
use crate::*;
use std::borrow::Cow;

/// Represents a text with formatting options.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Text<'a> {
    /// The actual text content.
    pub(crate) text: &'a str,
    /// The color of the text.
    pub(crate) text_color: ColorType,
    /// The background color of the text.
    pub(crate) text_bg_color: ColorType,
    /// Whether the text should be bold.
    pub(crate) blod: bool,
    /// endl
    pub(crate) endl: bool,
}

impl<'a> Default for Text<'a> {
    fn default() -> Self {
        Text {
            text: "",
            text_color: ColorType::default(),
            text_bg_color: ColorType::default(),
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
        let text_color: &String = &self.text_color.to_string();
        let text_bg_color: &String = &self.text_bg_color.get_str(DisplayType::Background);
        let mut colored_text: String = if blod {
            format!(
                "{}{}{}{}{}{}",
                text_bg_color, text_color, BLOD, text, UNBLOD, RESET
            )
        } else {
            format!("{}{}{}{}", text_bg_color, text_color, text, RESET)
        };
        if self.endl {
            colored_text.push_str("\n");
        }
        Cow::Owned(colored_text)
    }
}
