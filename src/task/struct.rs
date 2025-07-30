use crate::*;

/// Represents a collection of text tasks to be executed sequentially.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Task<'a> {
    /// Collection of text configurations to process
    pub(crate) text_list: Vec<Text<'a>>,
}
