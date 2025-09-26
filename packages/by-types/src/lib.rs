pub mod api_error;
pub mod auth;
pub mod config;
pub mod json_with_headers;
pub mod query_param;
pub mod query_response;

pub use api_error::*;
pub use auth::*;
pub use config::*;
pub use query_param::*;
pub use query_response::*;

pub use json_with_headers::*;

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WrappedType {
    Bigint(i64),
    Integer(i32),
    Text(String),
    Boolean(bool),
    Jsonb(serde_json::Value),
}

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    Eq,
    PartialEq,
    schemars::JsonSchema,
    aide::OperationIo,
)]
#[serde(rename_all = "snake_case")]
pub enum Conditions {
    BetweenBigint(String, i64, i64),
    EqualsBigint(String, i64),
    NotEqualsBigint(String, i64),
    GreaterThanBigint(String, i64),
    LessThanBigint(String, i64),
    GreaterThanEqualsBigint(String, i64),
    LessThanEqualsBigint(String, i64),

    BetweenInteger(String, i32, i32),
    EqualsInteger(String, i32),
    NotEqualsInteger(String, i32),
    GreaterThanInteger(String, i32),
    LessThanInteger(String, i32),
    GreaterThanEqualsInteger(String, i32),
    LessThanEqualsInteger(String, i32),

    EqualsText(String, String),
    NotEqualsText(String, String),
    ContainsText(String, String),
    NotContainsText(String, String),
    StartsWithText(String, String),
    NotStartsWithText(String, String),
    EndsWithText(String, String),
    NotEndsWithText(String, String),

    TrueBoolean(String),
    FalseBoolean(String),

    Custom(String),
}

impl Conditions {
    pub fn to_binder(&self, i: i32) -> (String, i32) {
        let q = match self {
            Conditions::BetweenBigint(field, _, _) => {
                let q = format!("{} BETWEEN ${} AND ${}", field, i, i + 1);
                return (q, i + 2);
            }
            Conditions::EqualsBigint(field, _) => format!("{} = ${}", field, i),
            Conditions::NotEqualsBigint(field, _) => format!("{} != ${}", field, i),
            Conditions::GreaterThanBigint(field, _) => format!("{} > ${}", field, i),
            Conditions::LessThanBigint(field, _) => format!("{} < ${}", field, i),
            Conditions::GreaterThanEqualsBigint(field, _) => format!("{} >= ${}", field, i),
            Conditions::LessThanEqualsBigint(field, _) => format!("{} <= ${}", field, i),

            Conditions::BetweenInteger(field, _, _) => {
                let q = format!("{} BETWEEN ${} AND ${}", field, i, i + 1);
                return (q, i + 2);
            }
            Conditions::EqualsInteger(field, _) => format!("{} = ${}", field, i),
            Conditions::NotEqualsInteger(field, _) => format!("{} != ${}", field, i),
            Conditions::GreaterThanInteger(field, _) => format!("{} > ${}", field, i),
            Conditions::LessThanInteger(field, _) => format!("{} < ${}", field, i),
            Conditions::GreaterThanEqualsInteger(field, _) => format!("{} >= ${}", field, i),
            Conditions::LessThanEqualsInteger(field, _) => format!("{} <= ${}", field, i),

            Conditions::EqualsText(field, _) => format!("{} = ${}", field, i),
            Conditions::NotEqualsText(field, _) => format!("{} != ${}", field, i),
            Conditions::ContainsText(field, _) => format!("{} ILIKE ${}", field, i),
            Conditions::NotContainsText(field, _) => format!("{} NOT ILIKE ${}", field, i),
            Conditions::StartsWithText(field, _) => format!("{} ILIKE ${}", field, i),
            Conditions::NotStartsWithText(field, _) => format!("{} NOT ILIKE ${}", field, i),
            Conditions::EndsWithText(field, _) => format!("{} ILIKE ${}", field, i),
            Conditions::NotEndsWithText(field, _) => format!("{} NOT ILIKE ${}", field, i),

            Conditions::TrueBoolean(field) => {
                let q = format!("{} = true", field);
                return (q, i);
            }
            Conditions::FalseBoolean(field) => {
                let q = format!("{} = false", field);
                return (q, i);
            }
            Conditions::Custom(q) => return (q.clone(), i),
        };

        (q, i + 1)
    }
}

#[derive(
    Debug,
    Default,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    Eq,
    PartialEq,
    schemars::JsonSchema,
    aide::OperationIo,
)]
#[serde(rename_all = "snake_case")]
pub enum Order {
    Asc(Vec<String>),
    Desc(Vec<String>),
    Random,
    #[default]
    None,
}

impl std::fmt::Display for Order {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Order::Asc(field) => format!("ORDER BY {} ASC", field.join(", ")),
            Order::Desc(field) => format!("ORDER BY {} DESC", field.join(", ")),
            Order::Random => format!("ORDER BY RANDOM()"),
            Order::None => "".to_string(),
        };

        write!(f, "{}", s)
    }
}
