use aws_config::BehaviorVersion;
use aws_sdk_bedrockagentruntime::{
    types::{
        FilterAttribute, KnowledgeBaseQuery, KnowledgeBaseRetrievalConfiguration,
        KnowledgeBaseVectorSearchConfiguration, RetrievalFilter,
    },
    Client as BedrockAgentClient,
};
use aws_sdk_bedrockruntime::types::{ContentBlock, ConversationRole, Message};
use aws_sdk_bedrockruntime::Client as BedrockClient;
use aws_smithy_types::Document;
use common::tracing::{debug, info};
use dioxus::prelude::ServerFnError;

const MAX_SECTION_CONTEXT_CHARS: usize = 120_000;

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
pub async fn build_bedrock_client() -> Result<BedrockClient, ServerFnError> {
    let aws_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    Ok(BedrockClient::new(&aws_config))
}

pub async fn build_bedrock_agent_client() -> Result<BedrockAgentClient, ServerFnError> {
    let aws_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    Ok(BedrockAgentClient::new(&aws_config))
}

pub async fn generate_section_html_kb(
    space_pk: &str,
    section_title: &str,
    section_focus: &str,
) -> Result<String, ServerFnError> {
    let config = crate::config::server_config::get();

    let filter = RetrievalFilter::Equals(
        FilterAttribute::builder()
            .key("space_pk")
            .value(Document::String(space_pk.to_string()))
            .build()
            .map_err(|e| ServerFnError::new(format!("kb filter build failed: {e:?}")))?,
    );

    let retrieval_config = KnowledgeBaseRetrievalConfiguration::builder()
        .vector_search_configuration(
            KnowledgeBaseVectorSearchConfiguration::builder()
                .filter(filter)
                .number_of_results(12)
                .build(),
        )
        .build();

    let agent = build_bedrock_agent_client().await?;
    let query = format!("{section_title} {section_focus} 관련 공공정책·여론조사 일반론 요약");
    let retrieval_query = KnowledgeBaseQuery::builder().text(query).build();
    let retrieved = agent
        .retrieve()
        .knowledge_base_id(config.bedrock_knowledge_base_id)
        .retrieval_query(retrieval_query)
        .retrieval_configuration(retrieval_config)
        .send()
        .await
        .map_err(|e| ServerFnError::new(format!("bedrock retrieve failed: {e:?}")))?;

    let retrieved = if retrieved.retrieval_results().is_empty() {
        info!("kb retrieve: 0 results with space_pk filter, retrying without filter");
        let no_filter_config = KnowledgeBaseRetrievalConfiguration::builder()
            .vector_search_configuration(
                KnowledgeBaseVectorSearchConfiguration::builder()
                    .number_of_results(12)
                    .build(),
            )
            .build();
        agent
            .retrieve()
            .knowledge_base_id(config.bedrock_knowledge_base_id)
            .retrieval_query(
                KnowledgeBaseQuery::builder()
                    .text(format!(
                        "{section_title} {section_focus} 관련 공공정책·여론조사 일반론 요약"
                    ))
                    .build(),
            )
            .retrieval_configuration(no_filter_config)
            .send()
            .await
            .map_err(|e| {
                ServerFnError::new(format!("bedrock retrieve failed (no filter): {e:?}"))
            })?
    } else {
        retrieved
    };

    let mut raw_context = String::new();
    for result in retrieved.retrieval_results() {
        if let Some(content) = result.content() {
            raw_context.push_str(content.text());
            raw_context.push('\n');
        }
    }

    info!(
        "kb retrieve: results={} raw_chars={}",
        retrieved.retrieval_results().len(),
        raw_context.chars().count()
    );
    if !raw_context.is_empty() {
        let sample: String = raw_context.chars().take(300).collect();
        debug!("kb retrieve sample: {}", sample);
    }

    info!(
        "kb sanitize: sanitized_chars={}",
        raw_context.chars().count()
    );
    if !raw_context.is_empty() {
        let sample: String = raw_context.chars().take(300).collect();
        debug!("kb sanitized sample: {}", sample);
    }
    let snapshot_payload = truncate_context(&raw_context);

    let prompt = format!(
        "You are generating one section of a report.\n\
Return ONLY valid HTML fragments (no markdown, no code fences/backticks).\n\
Do not include literal \"\\\\n\" sequences or any stray text outside HTML tags.\n\
Use a formal report tone in Korean (보고서형 문체, 객관적/분석적).\n\
Safety constraints:\n\
- Only discuss high-level, neutral policy/process considerations and anonymized aggregate trends.\n\
- Never fabricate numbers, percentages, dates, counts, or comparisons.\n\
Only use numeric facts that appear in the provided Context.\n\
If the Context lacks numeric evidence for a claim, write \"해당 데이터 없음\" instead of inventing numbers.\n\
Structure must be:\n\
<section>\n\
  <h1>SECTION TITLE</h1>\n\
  <ol>\n\
    <li><strong>Top Subheading</strong>\n\
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
  </ol>\n\
</section>\n\
Use the top-level subheadings EXACTLY as provided in \"Subheadings\" (same order).\n\
If a top-level subheading includes child items in parentheses, treat those as required sub-subheadings in the nested list.\n\
Do NOT include numbering text like \"1.\" inside the <strong> text (the <ol> handles numbering).\n\
Each top-level item must contain a nested <ul> of sub-subheadings.\n\
Each sub-subheading must contain a nested <ul> with AT LEAST 3 bullet sentences.\n\
Each bullet detail must be a complete sentence and end with a period.\n\
Each bullet should be at least 80 characters and include analysis or reasoning, not just listing.\n\
When possible, include numeric facts (counts, ratios, dates) drawn from the context.\n\
Never fabricate numbers, percentages, dates, counts, or comparisons.\n\
Only use numeric facts that appear in the provided Context.\n\
If the Context lacks numeric evidence for a claim, write \"해당 데이터 없음\" instead of inventing numbers.\n\
If data is missing, still include the subheading and write a bullet like \"해당 데이터 없음\".\n\
If a subheading includes conditions like \"성별 정보 존재 시\" or \"나이 정보 존재 시\", omit that subheading entirely when the data is not available.\n\
Add at least one <table> if the data supports it.\n\
Section Title: {section_title}\n\
Subheadings: {section_focus}\n\
Space PK: {space_pk}\n\
Context:\n\
{snapshot_payload}"
    );

    let message = Message::builder()
        .role(ConversationRole::User)
        .content(ContentBlock::Text(prompt))
        .build()
        .map_err(|e| ServerFnError::new(format!("bedrock message build failed: {e:?}")))?;

    let client = build_bedrock_client().await?;
    let response = client
        .converse()
        .model_id(config.bedrock_model_id)
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

    html = strip_code_fences(&html)
        .replace("\\n", "")
        .replace("\\t", "")
        .replace("\\r", "")
        .replace('\n', "")
        .replace('\r', "")
        .replace('\t', "");

    if html.trim().is_empty() {
        return Err(ServerFnError::new("kb returned empty html"));
    }

    Ok(html)
}
