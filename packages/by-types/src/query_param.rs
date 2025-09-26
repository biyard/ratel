#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    aide::OperationIo,
)]
pub enum QueryParam<T> {
    #[default]
    None,
    Some(T),
}

impl<T: std::fmt::Display> std::fmt::Display for QueryParam<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, ""),
            Self::Some(v) => write!(f, "{}", v),
        }
    }
}

impl<T: std::str::FromStr> std::str::FromStr for QueryParam<T> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Ok(Self::None);
        }

        match T::from_str(s) {
            Ok(v) => Ok(Self::Some(v)),
            Err(_) => Ok(Self::None),
        }
    }
}

impl<T: std::fmt::Display> QueryParam<T> {
    pub fn to_string(&self) -> String {
        format!("{}", self)
    }
}
