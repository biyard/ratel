use crate::common::ai::writer::{WriterAi, WriterAiError, WriterAiRequest};

/// Deterministic test writer. Returns a valid 5-section opinion-gathering
/// JSON response for any input. Only compiled under `bypass` feature.
#[derive(Default)]
pub struct FixtureWriter;

#[async_trait::async_trait]
impl WriterAi for FixtureWriter {
    async fn generate(
        &self,
        req: WriterAiRequest,
    ) -> std::result::Result<String, WriterAiError> {
        let language = if req.user_prompt.to_lowercase().contains("language: en")
            || req.user_prompt.to_lowercase().contains("language: english")
        {
            Lang::En
        } else {
            Lang::Ko
        };

        let (title, body) = match language {
            Lang::Ko => (
                "테스트 의견수렴 초안",
                "<h2>추진배경</h2><p>테스트 fixture 가 생성한 배경 문단입니다.</p>\
                 <h2>추진목적</h2><p>테스트 fixture 가 생성한 목적 문단입니다.</p>\
                 <h2>추진내용</h2><p>테스트 fixture 가 생성한 내용 문단입니다.</p>\
                 <h2>의견수렴 사항</h2><p>테스트 fixture 가 생성한 쟁점 문단입니다.</p>\
                 <h2>참여 안내</h2><p>테스트 fixture 가 생성한 참여 안내 문단입니다.</p>",
            ),
            Lang::En => (
                "Fixture Opinion Gathering Draft",
                "<h2>Background</h2><p>Fixture-generated background paragraph.</p>\
                 <h2>Purpose</h2><p>Fixture-generated purpose paragraph.</p>\
                 <h2>Content</h2><p>Fixture-generated content paragraph.</p>\
                 <h2>Topics for Input</h2><p>Fixture-generated topics paragraph.</p>\
                 <h2>How to Participate</h2><p>Fixture-generated participation paragraph.</p>",
            ),
        };

        Ok(format!(
            r#"{{"title":"{}","body_html":"{}"}}"#,
            title.replace('"', "\\\""),
            body.replace('"', "\\\"")
        ))
    }
}

enum Lang {
    Ko,
    En,
}
