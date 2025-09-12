use by_macros::EnumProp;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, EnumProp)]
pub enum Prop {
    Test,
    TestCase,
}

#[test]
fn test_enum_prop() {
    let _ = tracing_subscriber::fmt::try_init();
    assert_eq!(Prop::Test.to_string(), "test");
    assert_eq!(Prop::TestCase.to_string(), "test-case");
    assert_eq!(Prop::Test, Prop::from_str("test").unwrap());
    assert_eq!(Prop::TestCase, Prop::from_str("test-case").unwrap());
    assert_eq!(format!("{}", Prop::Test), "test");
    assert_eq!(format!("{}", Prop::TestCase), "test-case");
}
