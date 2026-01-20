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
    color::*, output::*, output_builder::*, output_list::*, output_list_builder::*, utils::*,
};

pub use hyperlane_time::*;

pub(crate) use {task::*, text::*};

pub(crate) use std::{
    borrow::Cow,
    fmt::{self, Display},
    io::Write,
    ops::Deref,
    slice::Iter,
};
