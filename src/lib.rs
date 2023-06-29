//! Serde JSON Canonicalization
//!
//! JSON Canonicalization Scheme [RFC-8785](https://datatracker.ietf.org/doc/html/rfc8785)
//! defines a JSON serialization scheme to allow using JSON data in
//! cryptographic operations that rely byte level reproduction of data. It can
//! be used instead of storing the serialized format as a BASE64 encoded string
//! or similar packaging, allowing easier handling of JSON formatted data that
//! can be canonicalized before feeding it to a cyrptographic function.
//!

mod jcs;

pub use crate::jcs::*;
