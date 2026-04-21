use crate::features::essence::types::*;
use crate::*;
use std::collections::HashSet;

/// Shared state + mutations for the Essence Sources page. Follows the
/// Ratel hook convention (see `features/me/hooks/use_my_spaces.rs`):
/// one `try_use_context` early-return, one `use_context_provider` at
/// the end. Mutations are exposed as `Callback<_>` fields so callers
/// write `hook.toggle_in_house.call(id)` without importing helpers.
///
/// `sources` is a plain `Signal<Vec<_>>` because the backend endpoint
/// isn't built yet. Once it lands, flip this to
/// `Loader<ListResponse<EssenceSourceResponse>>` and the return type to
/// `Result<UseEssenceSources, Loading>`.
#[derive(Clone, Copy, DioxusController)]
pub struct UseEssenceSources {
    pub sources: Signal<Vec<EssenceSourceResponse>>,
    pub selected_kind: Signal<KindFilter>,
    pub status_filter: Signal<StatusFilter>,
    pub sort_order: Signal<SortOrder>,
    pub search_query: Signal<String>,
    /// Set of source ids currently checkbox-selected for bulk actions.
    pub selected_rows: Signal<HashSet<String>>,
    /// Flip a single row's in-House toggle between On ↔ Off. Leaves
    /// Paused untouched (the paused rail is managed by `bulk_pause`).
    pub toggle_in_house: Callback<String>,
    pub bulk_pause: Callback<()>,
    pub bulk_reembed: Callback<()>,
    pub bulk_flag_ai: Callback<()>,
    pub bulk_remove: Callback<()>,
}

#[track_caller]
pub fn use_essence_sources() -> UseEssenceSources {
    let ctx: Option<UseEssenceSources> = try_use_context();
    if let Some(ctx) = ctx {
        return ctx;
    }

    let sources = use_signal(mock_sources);
    let selected_kind = use_signal(KindFilter::default);
    let status_filter = use_signal(StatusFilter::default);
    let sort_order = use_signal(SortOrder::default);
    let search_query = use_signal(String::new);
    let selected_rows: Signal<HashSet<String>> = use_signal(HashSet::new);

    let mut sources_for_toggle = sources;
    let toggle_in_house = use_callback(move |id: String| {
        sources_for_toggle.with_mut(|list| {
            for s in list.iter_mut() {
                if s.id == id {
                    s.in_house = match s.in_house {
                        InHouseStatus::On => InHouseStatus::Off,
                        InHouseStatus::Off => InHouseStatus::On,
                        InHouseStatus::Paused => InHouseStatus::Paused,
                    };
                    break;
                }
            }
        });
    });

    // Bulk actions operate on every source whose id is in `selected_rows`.
    // Each one walks the vec once and clears the selection at the end so
    // the bulk bar hides itself (matches the mockup's JS behaviour).
    let mut sources_for_bulk = sources;
    let mut selected_for_bulk = selected_rows;

    let bulk_pause = use_callback(move |_| {
        let ids: HashSet<String> = selected_for_bulk.read().clone();
        sources_for_bulk.with_mut(|list| {
            for s in list.iter_mut() {
                if ids.contains(&s.id) {
                    s.in_house = InHouseStatus::Paused;
                }
            }
        });
        selected_for_bulk.write().clear();
    });

    let bulk_reembed = use_callback(move |_| {
        // Client-side only for now — would kick a backend re-embed job.
        selected_for_bulk.write().clear();
    });

    let bulk_flag_ai = use_callback(move |_| {
        let ids: HashSet<String> = selected_for_bulk.read().clone();
        sources_for_bulk.with_mut(|list| {
            for s in list.iter_mut() {
                if ids.contains(&s.id) {
                    s.ai_flagged = true;
                }
            }
        });
        selected_for_bulk.write().clear();
    });

    let bulk_remove = use_callback(move |_| {
        let ids: HashSet<String> = selected_for_bulk.read().clone();
        sources_for_bulk.with_mut(|list| list.retain(|s| !ids.contains(&s.id)));
        selected_for_bulk.write().clear();
    });

    use_context_provider(move || UseEssenceSources {
        sources,
        selected_kind,
        status_filter,
        sort_order,
        search_query,
        selected_rows,
        toggle_in_house,
        bulk_pause,
        bulk_reembed,
        bulk_flag_ai,
        bulk_remove,
    })
}

/// Mock data mirroring the seven rows in `essence-sources.html`. Replace
/// with a real server call once the backend endpoint exists.
fn mock_sources() -> Vec<EssenceSourceResponse> {
    vec![
        EssenceSourceResponse {
            id: "src-mcp-essay".into(),
            kind: EssenceSourceKind::Notion,
            title: "Why MCP changes everything for knowledge creators".into(),
            source_path: "Notion · /workspace/essay/mcp".into(),
            chunks: 6,
            extra_meta: None,
            word_count: 1_340,
            last_synced_label: "2m ago".into(),
            quality_score: 4.8,
            in_house: InHouseStatus::On,
            ai_flagged: false,
        },
        EssenceSourceResponse {
            id: "src-decent-review".into(),
            kind: EssenceSourceKind::RatelPost,
            title: "Three years of decentralization arguments, reviewed".into(),
            source_path: "Ratel post · /p/decent-review".into(),
            chunks: 23,
            extra_meta: Some("142 likes".into()),
            word_count: 5_210,
            last_synced_label: "3d ago".into(),
            quality_score: 4.9,
            in_house: InHouseStatus::On,
            ai_flagged: false,
        },
        EssenceSourceResponse {
            id: "src-gov-token".into(),
            kind: EssenceSourceKind::Notion,
            title: "Notes on governance token design (2026 draft)".into(),
            source_path: "Notion · /workspace/drafts/gov-token".into(),
            chunks: 12,
            extra_meta: None,
            word_count: 2_870,
            last_synced_label: "18m ago".into(),
            quality_score: 4.6,
            in_house: InHouseStatus::On,
            ai_flagged: false,
        },
        EssenceSourceResponse {
            id: "src-qv-reply".into(),
            kind: EssenceSourceKind::Comment,
            title: "Reply on \"Why quadratic voting fails in practice\"".into(),
            source_path: "Ratel comment · /p/qv-fails#c42".into(),
            chunks: 2,
            extra_meta: None,
            word_count: 420,
            last_synced_label: "yesterday".into(),
            quality_score: 3.7,
            in_house: InHouseStatus::On,
            ai_flagged: false,
        },
        EssenceSourceResponse {
            id: "src-interview-prep".into(),
            kind: EssenceSourceKind::Notion,
            title: "Interview prep — Ratel Series A (draft)".into(),
            source_path: "Notion · /workspace/interview-prep".into(),
            chunks: 2,
            extra_meta: Some("Paused".into()),
            word_count: 460,
            last_synced_label: "yesterday".into(),
            quality_score: 3.4,
            in_house: InHouseStatus::Paused,
            ai_flagged: false,
        },
        EssenceSourceResponse {
            id: "src-treasury-vote".into(),
            kind: EssenceSourceKind::Action,
            title: "Vote: Ratel Treasury Allocation Q2 — Yes with reasoning".into(),
            source_path: "Ratel action · /spaces/treasury/poll-q2".into(),
            chunks: 1,
            extra_meta: None,
            word_count: 180,
            last_synced_label: "2d ago".into(),
            quality_score: 4.1,
            in_house: InHouseStatus::On,
            ai_flagged: false,
        },
        EssenceSourceResponse {
            id: "src-weekly-rituals".into(),
            kind: EssenceSourceKind::Notion,
            title: "Weekly rituals — productivity experiments".into(),
            source_path: "Notion · /workspace/rituals".into(),
            chunks: 4,
            extra_meta: Some("AI flagged".into()),
            word_count: 930,
            last_synced_label: "5d ago".into(),
            quality_score: 2.3,
            in_house: InHouseStatus::Off,
            ai_flagged: true,
        },
    ]
}
