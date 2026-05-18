//! Reports list context — mirrors the `use_wall_context` turn-key
//! pattern from PR #1593: a single `use_loader` call resolves the page's
//! data, the resulting `Loader<T>` is wrapped in a `DioxusController`
//! struct, and the layout publishes it via `use_context_provider`.
//! Consumers (list view, future detail view) read it back through
//! `use_report_list_context()` without re-fetching.
//!
//! Once the real backend lands, swap the mock body of `load_reports`
//! for a real handler call — the consumer surface stays unchanged.

use crate::features::spaces::pages::report::types::*;
use crate::*;

#[derive(Clone, Copy, DioxusController)]
pub struct UseReportListContext {
    pub space_id: ReadSignal<SpacePartition>,
    pub reports: Loader<Vec<ReportListItem>>,
}

impl UseReportListContext {
    pub fn drafts(&self) -> Vec<ReportListItem> {
        self.reports()
            .into_iter()
            .filter(|r| r.status == ReportStatus::Draft)
            .collect()
    }

    pub fn published(&self) -> Vec<ReportListItem> {
        self.reports()
            .into_iter()
            .filter(|r| r.status == ReportStatus::Published)
            .collect()
    }
}

#[track_caller]
pub fn use_report_list_context() -> UseReportListContext {
    use_context()
}

#[track_caller]
pub fn use_report_list_context_provider(
    space_id: ReadSignal<SpacePartition>,
) -> Result<UseReportListContext, Loading> {
    let reports =
        use_loader(move || async move { Ok::<_, crate::common::Error>(mock_reports()) })?;

    let ctx = use_context_provider(move || UseReportListContext { space_id, reports });

    Ok(ctx)
}

/// Local-only mock data — mirrors the four cards in
/// `assets/design/reports/reports-list.html`.
fn mock_reports() -> Vec<ReportListItem> {
    vec![
        ReportListItem {
            id: "rpt-2026q1-policy".to_string(),
            status: ReportStatus::Published,
            source: ReportSourceKind::Action,
            category: "Action · Report".to_string(),
            title: "2026 Q1 정책 우선순위 토론 분석".to_string(),
            description: "응답자 1,248명 기반 — 탄소 상쇄가 압도적 1위, 재생에너지가 2위로 이어지는 정책 우선순위 분포 정리.".to_string(),
            relative_time: "2d ago".to_string(),
        },
        ReportListItem {
            id: "rpt-follow-personas".to_string(),
            status: ReportStatus::Draft,
            source: ReportSourceKind::Follow,
            category: "Follow · Personas".to_string(),
            title: "참여자 페르소나 분석 (Follow Quest)".to_string(),
            description: "팔로워 2,104명을 4개 페르소나로 분류한 분석 보고서. 정책 관심 영역과 활동 시간대 패턴 포함.".to_string(),
            relative_time: "5h ago".to_string(),
        },
        ReportListItem {
            id: "rpt-quiz-pass-rate".to_string(),
            status: ReportStatus::Published,
            source: ReportSourceKind::Quiz,
            category: "Quiz · Pass Rate".to_string(),
            title: "Q1 퀴즈 통과율 — 정책 이해도 보고".to_string(),
            description: "10개 문항 × 684명 응시. 평균 통과율 67% — 가장 어려웠던 3개 문항과 오답 분포 분석.".to_string(),
            relative_time: "1w ago".to_string(),
        },
        ReportListItem {
            id: "rpt-poll-climate".to_string(),
            status: ReportStatus::Draft,
            source: ReportSourceKind::Poll,
            category: "Poll · Climate".to_string(),
            title: "탄소 상쇄 투표 결과 정리".to_string(),
            description: "3개 옵션 중 \"탄소 크레딧 거래\" 응답이 52%로 최다. 부문별 응답 분포 정리 중.".to_string(),
            relative_time: "방금".to_string(),
        },
    ]
}
