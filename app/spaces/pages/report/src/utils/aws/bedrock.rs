use aws_config::BehaviorVersion;
use aws_sdk_bedrockagentruntime::{
    types::{
        FilterAttribute, KnowledgeBaseQuery, KnowledgeBaseRetrievalConfiguration,
        KnowledgeBaseVectorSearchConfiguration, RetrievalFilter,
    },
    Client as BedrockAgentClient,
};
use aws_sdk_bedrockruntime::types::{
    ContentBlock, ConversationRole, InferenceConfiguration, Message,
};
use aws_sdk_bedrockruntime::Client as BedrockClient;
use aws_smithy_types::Document;
use common::tracing::{debug, info};
use dioxus::prelude::ServerFnError;

const MAX_SECTION_CONTEXT_CHARS: usize = 120_000;
const MIN_SUBSECTION_CHARS: usize = 1200;
const MAX_OUTPUT_TOKENS: i32 = 4096;

struct Bedrock;

impl Bedrock {
    async fn build_client() -> Result<BedrockClient, ServerFnError> {
        let aws_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
        Ok(BedrockClient::new(&aws_config))
    }

    async fn build_agent_client() -> Result<BedrockAgentClient, ServerFnError> {
        let aws_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
        Ok(BedrockAgentClient::new(&aws_config))
    }

    fn get_space_filter(space_pk: &str) -> Result<RetrievalFilter, ServerFnError> {
        Ok(RetrievalFilter::Equals(
            FilterAttribute::builder()
                .key("space_pk")
                .value(Document::String(space_pk.to_string()))
                .build()
                .map_err(|e| ServerFnError::new(format!("kb filter build failed: {e:?}")))?,
        ))
    }

    async fn query_kb(
        filter: RetrievalFilter,
        query: String,
    ) -> Result<Option<String>, ServerFnError> {
        let config = crate::config::server_config::get();
        let agent = Self::build_agent_client().await?;

        let retrieval_config = KnowledgeBaseRetrievalConfiguration::builder()
            .vector_search_configuration(
                KnowledgeBaseVectorSearchConfiguration::builder()
                    .filter(filter)
                    .number_of_results(10)
                    .build(),
            )
            .build();

        let retrieval_query = KnowledgeBaseQuery::builder().text(query).build();
        let retrieved = agent
            .retrieve()
            .knowledge_base_id(config.bedrock_knowledge_base_id)
            .retrieval_query(retrieval_query)
            .retrieval_configuration(retrieval_config)
            .send()
            .await
            .map_err(|e| ServerFnError::new(format!("bedrock retrieve failed: {e:?}")))?;

        if retrieved.retrieval_results().is_empty() {
            return Ok(None);
        }

        let mut raw_context = String::new();
        for result in retrieved.retrieval_results() {
            if let Some(content) = result.content() {
                raw_context.push_str(content.text());
                raw_context.push('\n');
            }
        }

        Ok(Some(raw_context))
    }
}

impl Bedrock {
    async fn query_chat(client: &BedrockClient, prompt: String) -> Result<String, ServerFnError> {
        let config = crate::config::server_config::get();
        let message = Message::builder()
            .role(ConversationRole::User)
            .content(ContentBlock::Text(prompt))
            .build()
            .map_err(|e| ServerFnError::new(format!("bedrock message build failed: {e:?}")))?;

        let response = client
            .converse()
            .model_id(config.bedrock_model_id)
            .inference_config(
                InferenceConfiguration::builder()
                    .max_tokens(MAX_OUTPUT_TOKENS)
                    .build(),
            )
            .messages(message)
            .send()
            .await
            .map_err(|e| ServerFnError::new(format!("bedrock converse failed: {e:?}")))?;

        let mut html = String::new();
        if let Some(output) = response.output() {
            if let Ok(message) = output.as_message() {
                for block in message.content() {
                    if let Ok(text) = block.as_text() {
                        html.push_str(text);
                    } else if let ContentBlock::Text(text) = block {
                        html.push_str(&text);
                    }
                }
            }
        }

        Ok(html)
    }

    async fn converse(prompt: String) -> Result<String, ServerFnError> {
        let client = Self::build_client().await?;
        Self::query_chat(&client, prompt).await
    }
}

fn truncate_context(snapshot_json: &str) -> String {
    if snapshot_json.chars().count() <= MAX_SECTION_CONTEXT_CHARS {
        return snapshot_json.to_string();
    }
    let mut truncated = snapshot_json
        .chars()
        .take(MAX_SECTION_CONTEXT_CHARS)
        .collect::<String>();
    truncated.push_str("\n\n[TRUNCATED]");
    truncated
}

fn strip_code_fences(raw: &str) -> String {
    let mut s = raw.trim().to_string();
    if s.starts_with("```") {
        // Remove opening fence line (``` or ```html).
        if let Some(pos) = s.find('\n') {
            s = s[(pos + 1)..].to_string();
        } else {
            return String::new();
        }
    }
    if s.ends_with("```") {
        s.truncate(s.len().saturating_sub(3));
    }
    s.trim().to_string()
}

pub async fn generate_subsection_html_kb(
    space_pk: &str,
    section_title: &str,
    subheading_title: &str,
    sub_subs: &[String],
) -> Result<String, ServerFnError> {
    let filter = Bedrock::get_space_filter(space_pk)?;

    let query = format!(
        "{section_title} {subheading_title} {} 관련 데이터 요약",
        if sub_subs.is_empty() {
            "".to_string()
        } else {
            format!("(하위: {})", sub_subs.join(", "))
        }
    );
    let raw_context = match Bedrock::query_kb(filter, query).await? {
        Some(result) => result,
        None => {
            info!("kb retrieve: 0 results for subsection {}", subheading_title);
            return Ok(String::new());
        }
    };
    let snapshot_payload = truncate_context(&raw_context);

    let subheadings_line = if sub_subs.is_empty() {
        "No sub-subheadings. Write bullet sentences directly under the subheading.".to_string()
    } else {
        format!(
            "Required sub-subheadings (use these exact titles, same order): {}",
            sub_subs.join(", ")
        )
    };

    let analysis_boost = analysis_detail_boost(section_title, subheading_title);
    let prompt = format!(
        "You are generating ONE list item for a report section.\n\
Return ONLY a single <li>...</li> fragment (no <section>, no <h1>, no <ol> wrapper).\n\
Do not include literal \"\\\\n\" sequences or any stray text outside HTML tags.\n\
Use a concise report tone in Korean using 음슴체 only (간결·객관적). Avoid \"~다\" endings.\n\
End every sentence with \"~함\", \"~됨\", \"~있음\", \"~없음\", or \"~임\".\n\
Safety constraints:\n\
- Never fabricate numbers, percentages, dates, counts, or comparisons.\n\
- Only use numeric facts that appear in the provided Context.\n\
- If data is missing for this subheading, return an empty string (no tags).\n\
Detail requirements:\n\
{analysis_boost}\n\
Table requirements:\n\
- Do NOT include any <table> in this output.\n\
Structure must be:\n\
<li><strong>SUBHEADING</strong>\n\
  <ul>\n\
    <li><strong>Sub-subheading</strong>\n\
      <ul>\n\
        <li>Detail sentence.</li>\n\
        <li>Detail sentence.</li>\n\
        <li>Detail sentence.</li>\n\
      </ul>\n\
    </li>\n\
  </ul>\n\
</li>\n\
If there are no sub-subheadings, then use:\n\
<li><strong>SUBHEADING</strong>\n\
  <ul>\n\
    <li>Detail sentence.</li>\n\
    <li>Detail sentence.</li>\n\
    <li>Detail sentence.</li>\n\
  </ul>\n\
</li>\n\
Each bullet must be a complete sentence ending with a period.\n\
Each bullet should be at least 110 characters.\n\
Use 3 to 5 bullets, aiming for 5 when Context supports it.\n\
{subheadings_line}\n\
Section Title: {section_title}\n\
Subheading: {subheading_title}\n\
Context:\n\
{snapshot_payload}"
    );

    let mut html = Bedrock::converse(prompt.clone()).await?;

    html = normalize_fragment_output(&html);
    if !is_valid_subsection_html(&html) || html.contains("<table") {
        for attempt in 0..2 {
            let retry_prompt = if attempt == 0 {
                format!(
                    "{prompt}\n\
RETRY: The previous output was incomplete or too short.\n\
Regenerate the entire <li> fragment from scratch.\n\
Do not truncate sentences, and do not leave any list items unfinished.\n\
Do NOT include any <table> elements."
                )
            } else {
                format!(
                    "{prompt}\n\
RETRY: Formatting rule violation. Regenerate following the rules strictly.\n\
Do NOT include any <table> elements."
                )
            };

            let retry_html = Bedrock::converse(retry_prompt).await?;
            html = normalize_fragment_output(&retry_html);
            if is_valid_subsection_html(&html) && !html.contains("<table") {
                break;
            }
        }
    }

    Ok(html)
}

fn normalize_fragment_output(raw: &str) -> String {
    strip_code_fences(raw)
        .replace("\\n", "")
        .replace("\\t", "")
        .replace("\\r", "")
        .replace('\n', "")
        .replace('\r', "")
        .replace('\t', "")
        .replace('\u{FFFD}', "")
        .trim()
        .to_string()
}

fn is_valid_subsection_html(html: &str) -> bool {
    if html.len() < MIN_SUBSECTION_CHARS {
        return false;
    }
    if !html.starts_with("<li") || !html.contains("</li>") {
        return false;
    }
    if !ends_with_complete_sentence(html) || !all_list_items_complete(html) {
        return false;
    }
    if contains_formal_ended(html) {
        return false;
    }
    if html.contains('\u{FFFD}') {
        return false;
    }
    true
}

fn contains_formal_ended(html: &str) -> bool {
    let mut text = String::new();
    let mut in_tag = false;
    for ch in html.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => text.push(ch),
            _ => {}
        }
    }
    text.contains("다.") || text.contains("다!") || text.contains("다?")
}

fn analysis_detail_boost(section_title: &str, subheading_title: &str) -> &'static str {
    if section_title == "분석 결과" && subheading_title.contains("토론 내용 분석") {
        "Provide richer analytical detail similar to a report summary. \
Use exactly 5 bullet sentences for each sub-subheading in this item. \
If the Context contains topic weights, coherence scores, centrality metrics, or keyword rankings, \
explicitly reference them and explain their implications. \
Explain contrasts between topics (e.g., which topic emphasizes which keywords) and connect to interpretation. \
Avoid vague statements like \"중요함\" without citing specific context evidence."
    } else if section_title == "분석 결과" {
        "Provide concrete analytical interpretations grounded in Context. \
Use exactly 5 bullet sentences for this item when possible. \
If specific metrics or rankings exist, cite them and explain their meaning. \
Avoid generic summaries without evidence."
    } else {
        "Provide concrete, evidence-based detail when Context supports it. Avoid generic filler."
    }
}

fn ends_with_complete_sentence(html: &str) -> bool {
    let mut text = String::new();
    let mut in_tag = false;
    for ch in html.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => text.push(ch),
            _ => {}
        }
    }
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return false;
    }
    matches!(
        trimmed.chars().last(),
        Some('.') | Some('!') | Some('?') | Some('。')
    )
}

fn all_list_items_complete(html: &str) -> bool {
    let mut parts = html.split("<li");
    let _ = parts.next();
    for part in parts {
        let Some(end) = part.find("</li>") else {
            return false;
        };
        let segment = &part[..end];
        let mut text = String::new();
        let mut in_tag = false;
        for ch in segment.chars() {
            match ch {
                '<' => in_tag = true,
                '>' => in_tag = false,
                _ if !in_tag => text.push(ch),
                _ => {}
            }
        }
        let trimmed = text.trim();
        if trimmed.is_empty() {
            return false;
        }
        if !matches!(
            trimmed.chars().last(),
            Some('.') | Some('!') | Some('?') | Some('。')
        ) {
            return false;
        }
    }
    true
}
