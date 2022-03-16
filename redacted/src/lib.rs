extern crate redacted_macro;

pub use redacted_macro::*;

pub trait Redact: std::fmt::Debug {}
