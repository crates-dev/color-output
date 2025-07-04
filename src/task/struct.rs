use crate::*;

/// Task structure
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Task<'a> {
    /// List of text structures
    pub(crate) text_list: Vec<Text<'a>>,
}
