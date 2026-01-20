pub(crate) mod r#fn;
pub(crate) mod r#impl;
pub(crate) mod r#struct;

#[cfg(test)]
mod test;

pub use r#struct::*;

pub(crate) use r#fn::*;
