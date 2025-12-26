use crate::*;

/// Trait for converting colors to their ANSI escape sequences.
pub(crate) trait ColorDisplay {
    /// Gets the ANSI escape sequence for the color.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Reference to self
    /// - `DisplayType` - Whether to apply to text or background
    ///
    /// # Returns
    ///
    /// - `String` - The ANSI escape sequence string
    fn get_str(&self, display_type: DisplayType) -> String;
}
