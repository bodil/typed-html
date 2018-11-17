#![feature(try_from)]

#[macro_use]
extern crate strum_macros;

pub extern crate htmlescape;
extern crate http;
extern crate language_tags;
extern crate mime;
extern crate stdweb;
extern crate strum;
extern crate typed_html_macros;

pub use typed_html_macros::html;

pub mod dom;
pub mod elements;
pub mod events;
pub mod types;

/// Marker trait for outputs
pub trait OutputType {}

/// String output
impl OutputType for String {}

/// DOM output
pub struct DOM;
impl OutputType for DOM {}
