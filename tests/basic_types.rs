use serde_json::{json, Value};
use test_case::test_case;

use serde_json_canonicalizer::to_vec;

// These do not play well with test_case macro
#[test]
fn literals() {
    assert_eq!(
        "null",
        String::from_utf8(to_vec(&json!(null)).unwrap()).unwrap()
    );
    assert_eq!(
        "true",
        String::from_utf8(to_vec(&json!(true)).unwrap()).unwrap()
    );
    assert_eq!(
        "false",
        String::from_utf8(to_vec(&json!(false)).unwrap()).unwrap()
    );
}

#[test_case(json!(42) => "42" ; "number")]
#[test_case(json!("42") => "\"42\"" ; "string")]
#[test_case(json!([]) => "[]" ; "empty array")]
#[test_case(json!({}) => "{}" ; "empty object")]
fn basic_types(value: Value) -> String {
    String::from_utf8(to_vec(&value).unwrap()).unwrap()
}

// This test case uses code from: https://github.com/probityai/jcs-admit/blob/main/tests/incumbent_differential.rs
#[test]
fn duplicate_key() {
    struct RepeatedMember;

    impl serde::Serialize for RepeatedMember {
        fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            use serde::ser::SerializeMap;
            let mut m = s.serialize_map(Some(2))?;
            m.serialize_entry("a", &1_u32)?;
            m.serialize_entry("a", &2_u32)?;
            m.end()
        }
    }

    let result =
        to_vec(&RepeatedMember).inspect_err(|err| assert!(err.to_string().contains("duplicate")));
    assert!(result.is_err());
}
