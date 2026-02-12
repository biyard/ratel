use common::serde_json::Value;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAIReportResponse {
    pub html_contents: String,
}

// FIXME: implement middleware and authorization
#[cfg(feature = "server")]
#[post("/v3/spaces/{space_pk}/analyze/ai-contents")]
pub async fn create_ai_report(space_pk: String) -> Result<CreateAIReportResponse, ServerFnError> {
    use common::serde_json;

    let config = crate::config::server_config::get();
    let base_prefix = format!("{}/spaces/{}/snapshots", config.env, space_pk);

    let s3 = super::utils::s3::build_s3_client().await?;
    let space_common =
        load_space_json(&s3, config.bucket_name, &base_prefix, "space_common.json").await?;
    let space_posts =
        load_space_json(&s3, config.bucket_name, &base_prefix, "space_posts.json").await?;
    let space_post_comments = load_space_json(
        &s3,
        config.bucket_name,
        &base_prefix,
        "space_post_comments.json",
    )
    .await?;
    let space_polls =
        load_space_json(&s3, config.bucket_name, &base_prefix, "space_polls.json").await?;
    let space_poll_user_answers = load_space_json(
        &s3,
        config.bucket_name,
        &base_prefix,
        "space_poll_user_answers.json",
    )
    .await?;
    let space_panel_participants = load_space_json(
        &s3,
        config.bucket_name,
        &base_prefix,
        "space_panel_participants.json",
    )
    .await?;
    let space_analyze_tfidf = load_space_json(
        &s3,
        config.bucket_name,
        &base_prefix,
        "space_analyze_tfidf.json",
    )
    .await?;
    let space_analyze_lda = load_space_json(
        &s3,
        config.bucket_name,
        &base_prefix,
        "space_analyze_lda.json",
    )
    .await?;
    let space_analyze_text_network = load_space_json(
        &s3,
        config.bucket_name,
        &base_prefix,
        "space_analyze_text_network.json",
    )
    .await?;

    let sections = vec![
        (
            "연구 개요",
            "연구 배경 및 필요성; 의제의 특성",
            serde_json::json!({
                "space_common": space_common.clone(),
                "space_posts": space_posts.clone(),
                "space_post_comments": space_post_comments.clone()
            }),
        ),
        (
            "조사 설계 및 데이터 수집",
            "조사 설계; 공론형 여론조사 진행 절차; 블록체인 기술 적용 방안; 분석 방법 및 데이터 수집 범위",
            serde_json::json!({
                "space_common": space_common.clone(),
                "space_polls": space_polls.clone(),
                "space_poll_user_answers": space_poll_user_answers.clone(),
                "space_panel_participants": space_panel_participants.clone()
            }),
        ),
        (
            "분석 결과",
            "설문 응답 분포 및 사전-사후 변화; 게시물/댓글 주요 논점; TF-IDF / LDA / Text Network 요약; 통합 분석",
            serde_json::json!({
                "space_posts": space_posts.clone(),
                "space_post_comments": space_post_comments.clone(),
                "space_polls": space_polls.clone(),
                "space_poll_user_answers": space_poll_user_answers.clone(),
                "space_analyze_tfidf": space_analyze_tfidf.clone(),
                "space_analyze_lda": space_analyze_lda.clone(),
                "space_analyze_text_network": space_analyze_text_network.clone()
            }),
        ),
        (
            "결론 및 제언",
            "연구 결과 요약; 의견 변화의 패턴과 원인; 방법론 및 정책적 제언",
            serde_json::json!({
                "space_common": space_common.clone(),
                "space_posts": space_posts.clone(),
                "space_post_comments": space_post_comments.clone(),
                "space_polls": space_polls.clone(),
                "space_poll_user_answers": space_poll_user_answers.clone(),
                "space_analyze_tfidf": space_analyze_tfidf.clone(),
                "space_analyze_lda": space_analyze_lda.clone(),
                "space_analyze_text_network": space_analyze_text_network.clone()
            }),
        ),
    ];

    let mut html_sections = Vec::with_capacity(sections.len());
    for (title, focus, context) in sections {
        let section_context_json = serde_json::to_string(&context)
            .map_err(|e| ServerFnError::new(format!("section json serialize failed: {e}")))?;
        let section_html = super::utils::bedrock::generate_section_html(
            &space_pk,
            title,
            focus,
            &section_context_json,
        )
        .await?;
        html_sections.push(section_html);
    }

    let html_contents = html_sections.join("");
    Ok(CreateAIReportResponse { html_contents })
}

async fn load_space_json(
    s3: &aws_sdk_s3::Client,
    bucket: &str,
    base_prefix: &str,
    filename: &str,
) -> Result<Value, ServerFnError> {
    let key = format!("{}/{}", base_prefix, filename);
    Ok(super::utils::s3::load_json_optional(s3, bucket, &key)
        .await?
        .unwrap_or(Value::Null))
}
