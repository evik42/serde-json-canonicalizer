[![Crates.io](https://img.shields.io/crates/v/serde_json_canonicalizer.svg)](https://crates.io/crates/serde_json_canonicalizer)

[![Workflow Status](https://github.com/evik42/serde-json-canonicalizer/workflows/nightly/badge.svg)](https://github.com/evik42/serde-json-canonicalizer/actions?query=workflow%3A%22nightly%22)
![Maintenance](https://img.shields.io/badge/maintenance-activly--developed-brightgreen.svg)

# serde_json_canonicalizer

Serde JSON Canonicalization

JSON Canonicalization Scheme [RFC-8785](https://datatracker.ietf.org/doc/html/rfc8785)
defines a JSON serialization scheme to allow using JSON data in
cryptographic operations that rely byte level reproduction of data. It can
be used instead of storing the serialized format as a BASE64 encoded string
or similar packaging, allowing easier handling of JSON formatted data that
can be canonicalized before feeding it to a cyrptographic function.


## License

Licensed under MIT license ([LICENSE](LICENSE) or https://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, shall be licensed as above,
without any additional terms or conditions.
