use crate::*;

/// Represents a collection of text tasks to be executed sequentially.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct Task<'a> {
    /// Collection of text configurations to process
    pub(crate) text_list: Vec<Text<'a>>,
}
