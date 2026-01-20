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

pub use {
    color::r#enum::*,
    output::{r#fn::*, r#struct::Output},
    output_builder::r#struct::OutputBuilder,
    output_list::r#struct::OutputList,
    output_list_builder::r#struct::OutputListBuilder,
    utils::r#fn::__println_text,
};

pub use hyperlane_time::*;

pub(crate) use {
    color::{r#const::*, r#fn::*, r#struct::*, r#trait::*},
    output_list::r#fn::*,
    task::r#struct::*,
    text::r#struct::*,
};

pub(crate) use std::{
    borrow::Cow,
    fmt::{self, Display},
    io::Write,
    ops::Deref,
    slice::Iter,
};
