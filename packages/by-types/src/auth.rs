use std::collections::HashMap;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, Eq, PartialEq, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: u64,
    pub role: Role,
    pub custom: HashMap<String, String>,
}

impl Claims {
    pub fn new(sub: String, role: Role) -> Self {
        Self {
            sub,
            role,
            ..Default::default()
        }
    }

    pub fn with_custom(mut self, key: String, value: String) -> Self {
        self.custom.insert(key, value);
        self
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    serde::Deserialize,
    serde::Serialize,
    Default,
    Eq,
    PartialEq,
    schemars::JsonSchema,
    aide::OperationIo,
)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Admin = 0,
    #[default]
    User = 1,
    // It means the user is not signed in web page.
    Guest = 10,
}

impl TryFrom<i32> for Role {
    type Error = String;

    fn try_from(value: i32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Role::Admin),
            1 => Ok(Role::User),
            10 => Ok(Role::Guest),
            _ => Err(format!("Invalid Role: {}", value)),
        }
    }
}

impl Into<i32> for Role {
    fn into(self) -> i32 {
        self as i32
    }
}

impl sqlx::Type<sqlx::Postgres> for Role {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <i32 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

impl sqlx::Encode<'_, sqlx::Postgres> for Role {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> std::result::Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let value: i32 = (*self).clone().into();
        <i32 as sqlx::Encode<sqlx::Postgres>>::encode_by_ref(&value, buf)
    }
}

impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Role {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> std::result::Result<Self, sqlx::error::BoxDynError> {
        let int_value: i32 = <i32 as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        Role::try_from(int_value)
            .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, e)).into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenScheme {
    Bearer,
    Usersig,
    XServerKey,
    Secret,
    Session,
}
impl std::fmt::Display for TokenScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenScheme::Bearer => write!(f, "Bearer"),
            TokenScheme::Usersig => write!(f, "Usersig"),
            TokenScheme::XServerKey => write!(f, "X-Server-Key"),
            TokenScheme::Secret => write!(f, "Secret"),
            TokenScheme::Session => write!(f, "Session"),
        }
    }
}

impl TryFrom<&str> for TokenScheme {
    type Error = String;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "bearer" => Ok(TokenScheme::Bearer),
            "usersig" => Ok(TokenScheme::Usersig),
            "x-server-key" => Ok(TokenScheme::XServerKey),
            "secret" => Ok(TokenScheme::Secret),
            "session" => Ok(TokenScheme::Session),
            _ => Err(format!("Invalid TokenScheme: {}", value)),
        }
    }
}
