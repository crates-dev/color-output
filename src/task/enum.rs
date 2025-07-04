use crate::*;

/// Result of a task operation
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum TaskResult<'a> {
    SuccessText(Text<'a>),
    SuccessStr(String),
    SuccessDefault,
    Fail,
}
