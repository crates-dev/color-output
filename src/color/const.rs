/// Default color (no formatting).
pub(crate) const DEFAULT: &str = "";

/// Semicolon separator used in CSI parameter lists.
pub(crate) const SEMICOLON: char = ';';

/// Control Sequence Introducer (CSI), starts an ANSI escape sequence.
pub(crate) const CSI: &str = "\x1b[";

/// Select Graphic Rendition (SGR) final byte.
pub(crate) const SGR: char = 'm';

/// Reset all graphic rendition attributes.
pub(crate) const SGR_RESET: &str = "\x1b[0m";

/// SGR parameter: bold or increased intensity.
pub(crate) const SGR_BOLD: &str = "1";

/// Line feed character.
pub(crate) const LINE_FEED: char = '\n';

/// ANSI escape code for black text.
pub(crate) const BLACK: &str = "30";

/// ANSI escape code for red text.
pub(crate) const RED: &str = "31";

/// ANSI escape code for green text.
pub(crate) const GREEN: &str = "32";

/// ANSI escape code for yellow text.
pub(crate) const YELLOW: &str = "33";

/// ANSI escape code for blue text.
pub(crate) const BLUE: &str = "34";

/// ANSI escape code for magenta text.
pub(crate) const MAGENTA: &str = "35";

/// ANSI escape code for cyan text.
pub(crate) const CYAN: &str = "36";

/// ANSI escape code for white text.
pub(crate) const WHITE: &str = "37";

/// ANSI escape code for black background.
pub(crate) const BG_BLACK: &str = "40";

/// ANSI escape code for red background.
pub(crate) const BG_RED: &str = "41";

/// ANSI escape code for green background.
pub(crate) const BG_GREEN: &str = "42";

/// ANSI escape code for yellow background.
pub(crate) const BG_YELLOW: &str = "43";

/// ANSI escape code for blue background.
pub(crate) const BG_BLUE: &str = "44";

/// ANSI escape code for magenta background.
pub(crate) const BG_MAGENTA: &str = "45";

/// ANSI escape code for cyan background.
pub(crate) const BG_CYAN: &str = "46";

/// ANSI escape code for white background.
pub(crate) const BG_WHITE: &str = "47";
