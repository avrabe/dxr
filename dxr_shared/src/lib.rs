//! # dxr_shared
//!
//! This crate provides base implementations of all XML-RPC types and functionality that is used in
//! the macros provided by `dxr_derive` and the high-level functionality provided in `dxr` itself.

#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(clippy::unwrap_used)]
#![cfg_attr(docsrs, feature(doc_cfg))]

// re-export of chrono, since it is part of the public API
pub use chrono;

mod ser_de;

mod error;
pub use error::*;

mod fault;
pub use fault::*;

mod from;
pub use from::*;

mod params;
pub use params::*;

mod to;
pub use to::*;

mod traits;
pub use traits::*;

mod types;
pub use types::{FaultResponse, MethodCall, MethodResponse, Value};

/// date & time format used by the XML-RPC `dateTime.iso8601` value type
pub const XML_RPC_DATE_FORMAT: &str = "%Y%m%dT%H:%M:%S";

// property-based (de)serialization tests
#[cfg(test)]
mod checks;

// standard (de)serialization tests
#[cfg(test)]
mod tests;