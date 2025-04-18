use crate::*;
use std::borrow::Cow;
use text::r#struct::Text;

/// Task structure
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Task<'a> {
    /// List of text structures
    pub(crate) text_list: Vec<Text<'a>>,
}
