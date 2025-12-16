[![Crates.io](https://img.shields.io/crates/v/serde_json_canonicalizer.svg)](https://crates.io/crates/serde_json_canonicalizer)
[![Workflow Status](https://github.com/evik42/serde-json-canonicalizer/actions/workflows/nightly.yml/badge.svg)](https://github.com/evik42/serde-json-canonicalizer/actions/workflows/nightly.yml)
![Maintenance](https://img.shields.io/badge/maintenance-activly--developed-brightgreen.svg)

# no-std branch

:warning: **This branch contains changes to make it work but are not officially supported**

This branch contains changes that allow building the library in a no-std environment. **Read carefully!**

Unfortunately [serde_json](https://crates.io/crates/serde_json) does not support implementing a custom Formatter in no-std environments because if seems that the `std` feature would not be additive to a no-std build. Details can be read on [serde_json PR #1122](https://github.com/serde-rs/json/pull/1122).

If you are determined to use this library in a no-std environment you will need to do it without relying on official releases from crates.io.

:warning: **Read this carefully**

Keep in mind that the patched version of `serde_json` can work in a `std` or in `no-std` environment, but there will be failures if you try to use a `no-std` build of a library where `serde_json` has `std` feature enabled.
The `no-std` branch of `serde_json_canonicalizer` **only builds with the patched version of serde_json**.

## Steps to get a no-std library version

1. Clone `serde_json`
2. Optionally checkout the release tag in `serde_json` that matches the version on crates.io that you want to use, or roll with master
3. Clone `serde_json_canonicalizer`
4. Switch `serde_json_canonicalizer` to this `no-std` branch
5. Apply the patch [serde-json-no-std.patch](serde-json-no-std.patch) to local copy of `serde_json`
   `git apply {serde-json-canonicalizer path}/serde-json-no-std.patch`
6. Use these local copies of the libraries to build your binary
7. Add `serde_json_canonicalizer` dependency in your Cargo.toml where appropriate
```
serde_json_canonicalizer = { version = "0.3", default-features = false, features = [ "alloc" ] }
```
8. In your binary or library Cargo.toml you need to override the path to both `serde_json` and `serde_json_canonicalizer` via the `patch.crates-io` directive
```
[patch.crates-io]
serde_json = { path = "/path/to/serde-json" }
serde_json_canonicalizer = { path = "../path/to/serde-json-canonicalizer" }
```

**At this point you are the maintainer of the version of both serde_json and serde_json_canonicalizer that you are using**

The beauty of free software.

In the [no-std-binary](no-std-binary) folder there is a little hacked example how a project in no-std would look like using these libraries.

**Normal README below**

# serde_json_canonicalizer

An RFC 8785 compatible JSON Canonicalization Scheme output for [serde_json](https://crates.io/crates/serde_json).

JSON Canonicalization Scheme [RFC-8785](https://datatracker.ietf.org/doc/html/rfc8785) defines a JSON serialization scheme to allow using JSON data in cryptographic operations that rely on byte level reproduction of data. It can be used instead of storing the serialized format as a BASE64 encoded string or similar packaging, allowing easier handling of JSON formatted data that can be canonicalized before feeding it to a cryptographic function.

## Usage
```rust
use serde_json_canonicalizer::{to_string, to_vec};

#[derive(serde::Serialize)]
struct Data {
    c: isize,
    b: bool,
    a: String,
}

let data = Data { c: 120, b: false, a: "Hello!".to_string() };
let expected = r#"{"a":"Hello!","b":false,"c":120}"#;

// serlialize to string or bytes, drop-in replacement for serde_json
let json_string = to_string(&data).unwrap();
let json_bytes = to_vec(&data).unwrap();

assert_eq!(json_string, expected);
assert_eq!(json_bytes, expected.as_bytes());
```

## serde_json arbitrary precision feature

`serde_json` supports writing arbitrary precision numbers into JSON which is not conforming to the RFC 8785. Those numbers can be reliably read back only by a deserializer that is prepared to read arbitrary precision numbers that are not available in JavaScript. The canonicalization method will convert these numbers into doubles and serialize them according to the rules of representing doubles. Thus the arbitrary precision is lost.
To use numbers that are not represented as doubles, store them as strings in the JSON and rely on the consuming application to deserialize these strings accordingly. (For example storing hash values, signatures or other numbers that cannot be represented in double format.)

## License

Licensed under MIT license ([LICENSE](LICENSE) or https://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, shall be licensed as above, without any additional terms or conditions.

## Compare to [serde_jcs](https://github.com/l1h3r/serde_jcs)

I created this crate because `serde_jcs` seems to be abandoned and the issues in that repository list a few things where it differs from the RFC. This crate aims to be 100% compatible with the RFC to be a suitable implementation in a multi-language environment.
