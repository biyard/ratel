use dioxus_translate::Translate;

#[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize, Translate, Clone)]
pub enum Error {
    #[error("serialize args: {0}")]
    #[translate(en = "Serialize error", ko = "Serialize error")]
    Serialize(String),

    #[error("deserialize result: {0}")]

    #[translate(en = "Deserialize error", ko = "Deserialize error")]
    Deserialize(String),
    #[error("command failed: {0}")]
    #[translate(en = "Deserialize error", ko = "Deserialize error")]
    CommandFailed(String),

    #[error("invalid url: {0}")]
    #[translate(en = "No found URL", ko = "URL을 찾을수 없습니다.")]
    InvalidUrl(String),
    #[error("opener failed: {0}")]
    #[translate(en = "Couldn't open URL", ko = "URL을 열지 못했습니다.")]
    OpenerFailed(String),
}
