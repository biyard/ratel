use crate::common::ListResponse;
use crate::common::hooks::{InfiniteQuery, use_infinite_query};
use crate::features::spaces::pages::apps::apps::analyzes::*;
use crate::*;

/// Controller hook for the Analyzes list page.
///
/// The polls query is always loaded. The discussions query is only
/// wired up under `feature = "local-dev"` because the corresponding
/// section is currently hidden in production builds — this matches the
/// existing gating in the pre-arena list (`views/mod.rs`). Keep the
/// cfg flags in lockstep on the struct field and the hook body so the
/// type stays consistent across build variants.
#[derive(Clone, Copy)]
pub struct UseSpaceAnalyzes {
    pub space_id: ReadSignal<SpacePartition>,
    pub polls: InfiniteQuery<String, AnalyzePollItem, ListResponse<AnalyzePollItem>>,
    #[cfg(feature = "local-dev")]
    pub discussions:
        InfiniteQuery<String, AnalyzeDiscussionItem, ListResponse<AnalyzeDiscussionItem>>,
}

#[track_caller]
pub fn use_space_analyzes(
    space_id: ReadSignal<SpacePartition>,
) -> std::result::Result<UseSpaceAnalyzes, RenderError> {
    if let Some(ctx) = try_use_context::<UseSpaceAnalyzes>() {
        return Ok(ctx);
    }

    let polls = use_infinite_query(move |bookmark| list_analyze_polls(space_id(), bookmark))?;

    #[cfg(feature = "local-dev")]
    let discussions =
        use_infinite_query(move |bookmark| list_analyze_discussions(space_id(), bookmark))?;

    Ok(use_context_provider(|| UseSpaceAnalyzes {
        space_id,
        polls,
        #[cfg(feature = "local-dev")]
        discussions,
    }))
}
