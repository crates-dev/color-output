use crate::*;
use std::borrow::Cow;
use text::r#type::Text;

/// Task structure
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Task<'a> {
    /// List of text structures
    pub(crate) text_list: Vec<Text<'a>>,
}

/// Result of a task operation
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum TaskResult<'a> {
    SuccessText(Text<'a>),
    SuccessStr(String),
    SuccessDefault,
    Fail,
}
