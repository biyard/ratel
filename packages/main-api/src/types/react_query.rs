#[derive(serde::Serialize)]
pub struct InitialQuery {
    key: serde_json::Value,
    data: serde_json::Value,
}

impl InitialQuery {
    pub fn new(
        key: impl serde::Serialize,
        value: impl serde::Serialize,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            key: serde_json::to_value(key)?,
            data: serde_json::to_value(value)?,
        })
    }

    pub fn new_infinite_list(
        key: impl serde::Serialize,
        value: impl serde::Serialize,
        bookmark: Option<String>,
    ) -> Result<Self, crate::Error> {
        let page = serde_json::to_value(value)?;

        Ok(Self {
            key: serde_json::to_value(key)?,
            data: serde_json::json!({
                "pages": [page],
                "pageParams": [bookmark]
            }),
        })
    }
}

#[derive(serde::Serialize)]
pub struct BootData {
    react_query: Vec<InitialQuery>,
}

impl BootData {
    pub fn new(react_query: Vec<InitialQuery>) -> Self {
        Self { react_query }
    }

    pub fn to_json(&self) -> Result<String, crate::Error> {
        Ok(serde_json::to_string(self)?.replace("</", "\\u003c/"))
    }
}
