use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAIReportResponse {
    pub html_contents: String,
}

// FIXME: implement middleware and authorization
#[post("/v3/spaces/{space_pk}/analyze/ai-contents")]
pub async fn create_ai_report(space_pk: String) -> Result<CreateAIReportResponse, ServerFnError> {
    let sections: Vec<(&str, Vec<&str>)> = vec![
        (
            "연구 개요",
            vec![
                "연구 배경 및 필요성 (하위: 현안의 대두 및 입법 동향, 쟁점의 복합성과 갈등 구조, 정책 결정의 딜레마와 숙고된 여론의 필요성[표 필요])",
                "의제의 특성",
                "조사 목적 (하위: 핵심 목적, 방법론적 혁신, 기대 성과)",
            ],
        ),
        (
            "조사 설계 및 데이터 수집",
            vec![
                "조사 설계",
                "여론조사 진행 절차 (하위: 온라인 의견 조사, 정보 제공, 비실시간 온라인 토의, 사후 온라인 의견 조사, 데이터 종합 분석)",
                "블록체인 기술 적용 방안 (하위: DID, 무결성 보장, 투명성 및 감사 가능성, 인센티브 지급 자동화/Smart Contract)",
                "분석 방법 (하위: 정량 분석, 정성 분석, 통합 분석)",
            ],
        ),
        (
            "분석 결과",
            vec![
                "공론형 여론조사: 의견 변화 분석 (하위: 전체 의견 변화 추이, 성별에 따른 의견 변화 비교[성별 정보 존재 시], 나이에 따른 의견 변화 비교[나이 정보 존재 시])",
                "사전 설문 문항 분석",
                "토론 내용 분석 (하위: 토론 분석_LDA, 토론 분석_TF-IDF, 토론 분석_Text Network, 통합 분석)",
            ],
        ),
        (
            "결론 및 제언",
            vec![
                "연구 결과 요약",
                "의견 변화의 패턴과 원인",
                "블록체인 기반 공론조사 방법론의 가능성",
            ],
        ),
    ];

    let mut html_sections = Vec::with_capacity(sections.len());
    let roman_titles = ["I.", "II.", "III.", "IV."];
    for (idx, (title, subheadings)) in sections.into_iter().enumerate() {
        let mut list_items = Vec::new();
        for subheading in subheadings {
            let item_html = crate::utils::aws::bedrock::generate_subsection_html_kb(
                &space_pk,
                title,
                subheading,
            )
            .await?;
            if !item_html.trim().is_empty() {
                list_items.push(item_html);
            }
        }
        let roman = roman_titles.get(idx).copied().unwrap_or("");
        let titled = if roman.is_empty() {
            title.to_string()
        } else {
            format!("{roman} {title}")
        };
        let section_html = format!(
            "<section><h1>{}</h1><ol>{}</ol></section>",
            titled,
            list_items.join("")
        );
        html_sections.push(section_html);
    }

    let html_contents = html_sections.join("");
    Ok(CreateAIReportResponse { html_contents })
}
