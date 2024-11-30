#![allow(warnings)]
mod color;
mod r#macro;
mod output;
mod output_builder;
mod output_list;
mod output_list_builder;
mod task;
mod text;
mod time;
pub use color::r#type::*;
pub use output::{output::*, r#type::Output};
pub use output_builder::r#type::OutputBuilder;
pub use output_list::r#type::OutputList;
pub use output_list_builder::r#type::OutputListBuilder;
pub use r#macro::proc_macro::*;
