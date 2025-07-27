//! color-output
//!
//! An atomic output library based on Rust that supports output
//! functionalities through functions, builders, and other methods.
//! It allows customization of text and background colors.

mod color;
mod r#macro;
mod output;
mod output_builder;
mod output_list;
mod output_list_builder;
mod task;
mod text;

pub use color::r#type::*;
pub use hyperlane_time::*;
pub use output::{r#fn::*, r#struct::Output};
pub use output_builder::r#struct::OutputBuilder;
pub use output_list::r#struct::OutputList;
pub use output_list_builder::r#struct::OutputListBuilder;

pub(crate) use color::{r#const::*, r#fn::*, r#struct::*};
pub(crate) use output_list::r#fn::output_list;
pub(crate) use std::borrow::Cow;
pub(crate) use std::{
    fmt::{self, Display},
    io::Write,
};
pub(crate) use task::r#struct::*;
pub(crate) use text::r#struct::Text;
