use crate::common::*;
use crate::features::posts::controllers::list_user_drafts::list_team_drafts_handler;
use crate::features::timeline::components::DraftScrollRow;

/// A horizontal row of the team's draft posts, displayed at the top of the
/// team home page. Thin wrapper that fetches team drafts and delegates
/// rendering (cards, scroll affordances, delete) to [`DraftScrollRow`].
#[component]
pub fn TeamDraftTimeline(username: String) -> Element {
    let mut teamname_signal = use_signal(|| username.clone());
    use_effect(use_reactive((&username,), move |(name,)| {
        if *teamname_signal.peek() != name {
            teamname_signal.set(name);
        }
    }));

    let drafts = use_server_future(move || {
        let teamname = teamname_signal();
        async move {
            let result = list_team_drafts_handler(teamname, None).await;
            if let Err(ref e) = result {
                tracing::error!("Failed to load team drafts: {:?}", e);
            }
            result
        }
    })?;

    let val = drafts.read();
    let res = val.as_ref().unwrap();

    let items = match res {
        Ok(resp) => resp.items.clone(),
        Err(_) => vec![],
    };

    if items.is_empty() {
        return rsx! {};
    }

    rsx! {
        DraftScrollRow {
            items,
            aria_label: "Team Drafts section".to_string(),
            test_id: "team-draft-timeline".to_string(),
        }
    }
}
