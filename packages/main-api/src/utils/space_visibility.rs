use dto::{Space, SpaceStatus};

#[derive(Debug, Clone)]
pub struct ViewerCtx {
    pub user_id: i64,
    pub team_ids: Vec<i64>,
}

/// Applies the space visibility rules to determine if the viewer can see the space.
/// Rules:
/// - Drafts can only be viewed by the author or team members (no anonymous access).
/// - Non-drafts:
///   - PublishingScope::Public can be viewed by anyone (including anonymous users).
///   - Other scopes can only be viewed by the author or team members.
pub fn scope_space_for_viewer(space: Space, ctx: &ViewerCtx) -> Option<Space> {
    let is_anonymous = ctx.user_id == 0;
    let is_author = space.author.iter().any(|a| a.id == ctx.user_id);
    let is_team_author = space
        .author
        .iter()
        .any(|a| ctx.team_ids.iter().any(|&tid| tid == a.id));

    if space.status == SpaceStatus::Draft {
        return if !is_anonymous && (is_author || is_team_author) {
            Some(space)
        } else {
            None
        };
    }
    //TODO: This logic will be applied after further testing
    // Check Space PublishingScope and apply visibility rules
    // match space.publishing_scope {
    //     PublishingScope::Public => Some(space),
    //     _ => {
    //         if is_author || is_team_author {
    //             Some(space)
    //         } else {
    //             None
    //         }
    //     }
    // }
    Some(space)
}

pub fn scope_space_opt_to_vec(space: Option<Space>, ctx: &ViewerCtx) -> Vec<Space> {
    match space {
        Some(s) => scope_space_for_viewer(s, ctx)
            .map(|s| vec![s])
            .unwrap_or_default(),
        None => Vec::new(),
    }
}
