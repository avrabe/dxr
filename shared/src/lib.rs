//! # dxr_shared
//!
//! This crate provides base implementations of all XML-RPC types and functionality that is used in
//! the macros provided by `dxr_derive` and the high-level functionality provided in `dxr` itself.

#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(clippy::unwrap_used)]

pub use base64;
pub use chrono;

use std::borrow::Cow;

use thiserror::Error;

/// date & time format used by the XML-RPC `dateTime.iso8601` value type
pub const XML_RPC_DATE_FORMAT: &str = "%Y%m%dT%H:%M:%S";

mod ser_de;

mod from;
pub use from::*;

mod to;
pub use to::*;

mod types;
pub use types::*;

/// conversion trait from XML-RPC values to primitives, `Option`, `HashMap`, and user-defined types
pub trait FromValue<T> {
    /// fallible conversion method from an XML-RPC value into the target type
    ///
    /// If the value contains a type that is not compatible with the target type, the conversion
    /// will fail.
    fn from_value(value: &Value) -> Result<T, ValueError>;
}

/// conversion trait from primitives, `Option`, `HashMap`, and user-defined types to XML-RPC values
pub trait ToValue<T> {
    /// conversion method from types into XML-RPC values
    ///
    /// The resulting XML-RPC value will automatically have a compatible type, so this conversion
    /// cannot fail, as this trait can only implemented for applicable types.
    fn to_value(value: &T) -> Value;
}

#[derive(Debug, Error)]
/// error type used for conversion errors between XML-RPC values and Rust values
pub enum ValueError {
    /// error variant describing a missing struct field
    #[error("Missing struct field: {}", .name)]
    MissingField {
        /// name of the missing struct field
        name: Cow<'static, str>,
    },
    /// error variant describing a type mismatch between XML-RPC value and a Rust struct field
    #[error("Type mismatch: got {}, expected {}", .argument, .expected)]
    WrongType {
        /// mismatched input type
        argument: Cow<'static, str>,
        /// expected input type
        expected: Cow<'static, str>,
    },
}

impl ValueError {
    /// construct a [`ValueError`] for a missing struct field
    pub fn missing_field(name: &'static str) -> ValueError {
        ValueError::MissingField {
            name: Cow::Borrowed(name),
        }
    }

    /// construct a [`ValueError`] for a type mismatch
    pub fn wrong_type(argument: &'static str, expected: &'static str) -> ValueError {
        ValueError::WrongType {
            argument: Cow::Borrowed(argument),
            expected: Cow::Borrowed(expected),
        }
    }
}

#[cfg(test)]
mod tests;