use by_macros::QueryDisplay;
use serde::Serialize;

#[derive(QueryDisplay, Serialize)]
struct TestStruct {
    key: String,
    value: String,
}

#[derive(QueryDisplay, Serialize)]
struct EmptyStruct;

#[test]
fn test_query_display() {
    let test = TestStruct {
        key: "example".to_string(),
        value: "query".to_string(),
    };

    let expected = "key=example&value=query";

    assert_eq!(test.to_string(), expected);
}

#[test]
fn test_empty_struct() {
    let empty = EmptyStruct;

    let expected = "";

    assert_eq!(empty.to_string(), expected);
}

#[test]
fn test_special_characters() {
    let test = TestStruct {
        key: "key with spaces".to_string(),
        value: "value&special=chars".to_string(),
    };

    let expected = "key=key+with+spaces&value=value%26special%3Dchars";

    assert_eq!(test.to_string(), expected);
}
