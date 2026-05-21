//! Opinion-gathering AI draft service.
//!
//! Builds the LLM prompt, calls the configured `WriterAi`, strips the
//! response down to a JSON object, parses it, and verifies the resulting
//! HTML contains exactly the five expected sections. The post-edit
//! controller uses this; tests inject a mock `WriterAi` via
//! `generate_opinion_draft_with`.

use serde::{Deserialize, Serialize};

use crate::common::ai::{writer_ai, WriterAi, WriterAiRequest};
use crate::features::posts::types::{AiDraftLanguage, AiPostDraftError};

const MAX_TOKENS: i32 = 2048;
const TEMPERATURE: f32 = 0.4;

/// Section headings the prompt asks the model to use, language-keyed.
/// The same list is used to verify the response after parsing.
const SECTIONS_KO: [&str; 5] = [
    "추진배경",
    "추진목적",
    "추진내용",
    "의견수렴 사항",
    "참여 안내",
];
const SECTIONS_EN: [&str; 5] = [
    "Background",
    "Purpose",
    "Content",
    "Topics for Input",
    "How to Participate",
];

#[derive(Debug, Clone)]
pub struct OpinionDraftInput {
    pub topic: String,
    pub background: String,
    pub feedback_request: String,
    pub participation_notes: Option<String>,
    pub language: AiDraftLanguage,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpinionDraftOutput {
    pub title: String,
    pub body_html: String,
}

/// Production entry-point. Uses the globally selected `WriterAi`.
pub async fn generate_opinion_draft(
    input: OpinionDraftInput,
) -> std::result::Result<OpinionDraftOutput, AiPostDraftError> {
    generate_opinion_draft_with(writer_ai(), input).await
}

/// Dependency-injected variant used by tests.
pub async fn generate_opinion_draft_with(
    writer: &dyn WriterAi,
    input: OpinionDraftInput,
) -> std::result::Result<OpinionDraftOutput, AiPostDraftError> {
    let prompt = build_prompt(&input, /* strict = */ false);
    let req = WriterAiRequest {
        user_prompt: prompt.clone(),
        max_tokens: MAX_TOKENS,
        temperature: TEMPERATURE,
    };

    let raw = writer.generate(req).await.map_err(|e| {
        tracing::error!(error = ?e, "ai writer generate failed");
        AiPostDraftError::BedrockFailed
    })?;

    match parse_and_verify(&raw, input.language) {
        Ok(out) => Ok(out),
        Err(first_err) => {
            tracing::warn!(
                error = ?first_err,
                "ai draft parse failed on first attempt; retrying with stricter prompt"
            );
            let strict_prompt = build_prompt(&input, /* strict = */ true);
            let raw2 = writer
                .generate(WriterAiRequest {
                    user_prompt: strict_prompt,
                    max_tokens: MAX_TOKENS,
                    temperature: TEMPERATURE,
                })
                .await
                .map_err(|e| {
                    tracing::error!(error = ?e, "ai writer generate failed on retry");
                    AiPostDraftError::BedrockFailed
                })?;
            parse_and_verify(&raw2, input.language).map_err(|e| {
                tracing::error!(error = ?e, "ai draft parse failed on retry");
                AiPostDraftError::GenerationFailed
            })
        }
    }
}

fn build_prompt(input: &OpinionDraftInput, strict: bool) -> String {
    let (lang_name, sections) = match input.language {
        AiDraftLanguage::Ko => ("Korean", SECTIONS_KO),
        AiDraftLanguage::En => ("English", SECTIONS_EN),
    };
    let notes = input
        .participation_notes
        .clone()
        .unwrap_or_else(|| "(none provided)".to_string());

    let strict_lead = if strict {
        "RESPOND ONLY with a JSON object. No prose, no markdown fences, no leading or trailing text. If you cannot comply, return an empty JSON object {}.\n\n"
    } else {
        ""
    };

    format!(
        "{strict_lead}You are a writing assistant for Ratel, a public-deliberation platform. \
You help users draft \"opinion gathering\" posts that follow a strict 5-section structure. \
Use ONLY the information the user provides below. Do NOT invent facts, statistics, names, \
dates, or quotes. If a section has no input from the user, write a brief neutral placeholder \
asking the post author to fill it in.\n\n\
Respond ONLY with a JSON object. No prose, no markdown fences, no explanations.\n\n\
Output schema:\n\
{{\n\
  \"title\": \"<a clear post title in {lang_name}, 80 chars max>\",\n\
  \"body_html\": \"<exactly 5 sections in this order, each <h2>HEADING</h2><p>PARAGRAPHS</p>>\"\n\
}}\n\n\
Section headings (use exactly these strings, in {lang_name}):\n\
  1. {h1}\n  2. {h2}\n  3. {h3}\n  4. {h4}\n  5. {h5}\n\n\
Language: {lang_code}\n\n\
User inputs:\n\
  Topic: {topic}\n\
  Background: {background}\n\
  Feedback the author wants: {feedback}\n\
  Participation notes: {notes}\n",
        strict_lead = strict_lead,
        lang_name = lang_name,
        lang_code = input.language.as_code(),
        h1 = sections[0],
        h2 = sections[1],
        h3 = sections[2],
        h4 = sections[3],
        h5 = sections[4],
        topic = input.topic,
        background = input.background,
        feedback = input.feedback_request,
        notes = notes,
    )
}

#[derive(Debug)]
enum ParseError {
    NoJsonObject,
    InvalidJson(String),
    MissingSection(&'static str),
}

fn parse_and_verify(
    raw: &str,
    language: AiDraftLanguage,
) -> std::result::Result<OpinionDraftOutput, ParseError> {
    let json = extract_first_json_object(raw).ok_or(ParseError::NoJsonObject)?;
    let parsed: OpinionDraftOutput = serde_json::from_str(json)
        .map_err(|e| ParseError::InvalidJson(e.to_string()))?;

    let sections = match language {
        AiDraftLanguage::Ko => SECTIONS_KO,
        AiDraftLanguage::En => SECTIONS_EN,
    };
    for heading in sections {
        if !parsed.body_html.contains(heading) {
            return Err(ParseError::MissingSection(heading));
        }
    }
    Ok(parsed)
}

/// Returns the substring `&raw[start..end+1]` that contains the first
/// balanced top-level JSON object, or `None` if none was found. Handles
/// strings (`"..."`) so braces inside text don't break the count.
fn extract_first_json_object(raw: &str) -> Option<&str> {
    let bytes = raw.as_bytes();
    let mut start = None;
    let mut depth = 0i32;
    let mut in_string = false;
    let mut escape = false;
    for (i, &b) in bytes.iter().enumerate() {
        if in_string {
            if escape {
                escape = false;
            } else if b == b'\\' {
                escape = true;
            } else if b == b'"' {
                in_string = false;
            }
            continue;
        }
        match b {
            b'"' => in_string = true,
            b'{' => {
                if start.is_none() {
                    start = Some(i);
                }
                depth += 1;
            }
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    if let Some(s) = start {
                        return std::str::from_utf8(&bytes[s..=i]).ok();
                    }
                }
            }
            _ => {}
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_handles_leading_prose() {
        let raw = r#"Here you go: {"title":"x","body_html":"y"} thanks!"#;
        assert_eq!(
            extract_first_json_object(raw),
            Some(r#"{"title":"x","body_html":"y"}"#)
        );
    }

    #[test]
    fn extract_handles_braces_in_strings() {
        let raw = r#"{"title":"x { y } z","body_html":"y"}"#;
        assert_eq!(extract_first_json_object(raw), Some(raw));
    }

    #[test]
    fn parse_rejects_missing_section() {
        let raw = r#"{"title":"t","body_html":"<h2>추진배경</h2><p>p</p>"}"#;
        assert!(matches!(
            parse_and_verify(raw, AiDraftLanguage::Ko),
            Err(ParseError::MissingSection(_))
        ));
    }

    #[test]
    fn parse_accepts_all_5_sections_ko() {
        let body = "<h2>추진배경</h2><p>a</p>\
                    <h2>추진목적</h2><p>b</p>\
                    <h2>추진내용</h2><p>c</p>\
                    <h2>의견수렴 사항</h2><p>d</p>\
                    <h2>참여 안내</h2><p>e</p>";
        let raw = format!(r#"{{"title":"t","body_html":"{}"}}"#, body);
        let result = parse_and_verify(&raw, AiDraftLanguage::Ko).unwrap();
        assert_eq!(result.title, "t");
        assert_eq!(result.body_html, body);
    }
}
