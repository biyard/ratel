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
            "연구 배경 및 필요성 (하위: 현안의 대두 및 입법 동향, 쟁점의 복합성과 갈등 구조, 정책 결정의 딜레마와 숙고된 여론의 필요성[표 필요]); 의제의 특성; 조사 목적 (하위: 핵심 목적, 방법론적 혁신, 기대 성과)",
            serde_json::json!({
                "space_common": space_common.clone(),
                "space_posts": space_posts.clone(),
                "space_post_comments": space_post_comments.clone()
            }),
        ),
        (
            "조사 설계 및 데이터 수집",
            "조사 설계; 여론조사 진행 절차 (온라인 의견 조사 -> 정보 제공 -> 비실시간 온라인 토의 -> 사후 온라인 의견 조사 -> 데이터 종합 분석); 블록체인 기술 적용 방안 (DID, 무결성 보장, 투명성 및 감사 가능성, 인센티브 지급 자동화/Smart Contract); 분석 방법 (정량 분석, 정성 분석, 통합 분석)",
            serde_json::json!({
                "space_common": space_common.clone(),
                "space_polls": space_polls.clone(),
                "space_poll_user_answers": space_poll_user_answers.clone(),
                "space_panel_participants": space_panel_participants.clone()
            }),
        ),
        (
            "분석 결과",
            "공론형 여론조사: 의견 변화 분석 (하위: 전체 의견 변화 추이, 성별에 따른 의견 변화 비교[성별 정보 존재 시], 나이에 따른 의견 변화 비교[나이 정보 존재 시]); 사전 설문 문항 분석; 토론 내용 분석 (하위: 토론 분석_LDA, 토론 분석_TF-IDF, 토론 분석_Text Network, 통합 분석)",
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
            "연구 결과 요약; 의견 변화의 패턴과 원인; 블록체인 기반 공론조사 방법론의 가능성",
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
