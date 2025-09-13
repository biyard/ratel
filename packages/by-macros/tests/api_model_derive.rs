use by_macros::ApiModel;

#[derive(ApiModel, Default, Debug, Clone, Eq, PartialEq)]
pub enum ApiModelTest {
    Admin = 0,
    #[default]
    User = 1,
    Guest = 10,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct WrappedApiModelTest {
    pub id: i64,
    pub role: ApiModelTest,
    pub name: String,
}

#[cfg(test)]
mod api_model_derive_tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialization() {
        let admin = ApiModelTest::Admin;
        let user = ApiModelTest::User;
        let guest = ApiModelTest::Guest;

        let admin_json = serde_json::to_string(&admin).unwrap();
        let user_json = serde_json::to_string(&user).unwrap();
        let guest_json = serde_json::to_string(&guest).unwrap();

        assert_eq!(admin_json, "0");
        assert_eq!(user_json, "1");
        assert_eq!(guest_json, "10");
    }

    #[test]
    fn test_deserialization() {
        let admin: ApiModelTest = serde_json::from_str("0").unwrap();
        let user: ApiModelTest = serde_json::from_str("1").unwrap();
        let guest: ApiModelTest = serde_json::from_str("10").unwrap();

        assert_eq!(admin, ApiModelTest::Admin);
        assert_eq!(user, ApiModelTest::User);
        assert_eq!(guest, ApiModelTest::Guest);
    }

    #[test]
    fn test_invalid_deserialization() {
        let invalid: Result<ApiModelTest, _> = serde_json::from_str("99");
        assert!(invalid.is_err());
    }

    #[test]
    fn test_wrapped_serialization() {
        let wrapped = WrappedApiModelTest {
            id: 123,
            role: ApiModelTest::Admin,
            name: "Alice".to_string(),
        };

        let json = serde_json::to_string(&wrapped).unwrap();
        assert_eq!(json, r#"{"id":123,"role":0,"name":"Alice"}"#);
    }

    #[test]
    fn test_wrapped_deserialization() {
        let json = r#"{"id":456,"role":1,"name":"Bob"}"#;
        let wrapped: WrappedApiModelTest = serde_json::from_str(json).unwrap();

        assert_eq!(wrapped.id, 456);
        assert_eq!(wrapped.role, ApiModelTest::User);
        assert_eq!(wrapped.name, "Bob");
    }

    #[test]
    fn test_invalid_role_deserialization() {
        let json = r#"{"id":789,"role":99,"name":"Charlie"}"#;
        let result: Result<WrappedApiModelTest, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
