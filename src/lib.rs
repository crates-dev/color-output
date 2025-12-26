//! color-output
//!
//! An atomic output library based on Rust that supports output
//! functionalities through functions, builders, and other methods.
//! It allows customization of text and background colors.

mod color;
mod output;
mod output_builder;
mod output_list;
mod output_list_builder;
mod task;
mod text;
mod utils;

pub use color::r#enum::*;
pub use hyperlane_time::*;
pub use output::{r#fn::*, r#struct::Output};
pub use output_builder::r#struct::OutputBuilder;
pub use output_list::r#struct::OutputList;
pub use output_list_builder::r#struct::OutputListBuilder;
pub use utils::r#fn::__println_text;

pub(crate) use color::{r#const::*, r#fn::*, r#struct::*, r#trait::*};
pub(crate) use output_list::r#fn::*;
pub(crate) use task::r#struct::*;
pub(crate) use text::r#struct::*;

pub(crate) use std::{
    borrow::Cow,
    fmt::{self, Display},
    io::Write,
    ops::Deref,
    slice::Iter,
};
