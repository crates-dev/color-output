use crate::*;
use std::borrow::Cow;
use text::r#struct::Text;

/// Result of a task operation
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum TaskResult<'a> {
    SuccessText(Text<'a>),
    SuccessStr(String),
    SuccessDefault,
    Fail,
}
