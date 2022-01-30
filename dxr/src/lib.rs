//! # dxr: declarative xml-rpc
//!
//! The `dxr` crate provides types, macros, and other functionality which can be used to write
//! fast and correct XML-API clients in Rust conveniently.

#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(clippy::unwrap_used)]

pub use dxr_derive::{FromValue, ToValue};
pub use dxr_shared::*;

#[cfg(test)]
mod tests;