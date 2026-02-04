//! color-output
//!
//! An atomic output library based on Rust that supports output
//! functionalities through functions, builders, and other methods.
//! It allows customization of text and background colors.

mod color;
mod task;
mod text;
mod utils;

pub use {color::*, utils::*};

pub use hyperlane_time::*;

use {task::*, text::*};

use std::{
    borrow::Cow,
    fmt::{self, Display},
    io::Write,
    ops::Deref,
    slice::Iter,
};
