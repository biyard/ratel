//! Hydration helper shared by `preview_analyze_report` (live sample)
//! and `list_analyze_records` (paginated detail). Takes a vec of
//! frozen `MatchedRecordRef`s and returns hydrated `AnalyzeRecordRow`s
//! ready for the table view.
//!
//! Per-page parent caching: each filter's source entity (Poll / Quiz /
//! Post) is loaded once even if the page contains many rows for that
//! filter. User info resolves in one `join_all` fan-out so the
//! wall-clock stays bounded by the slowest single GetItem rather than
//! the row count.
//!
//! Anonymous spaces (`SpaceCommon.anonymous_participation == true`)
//! resolve actor identity through `SpaceParticipant` instead of the
//! global `User` row — same anonymization the rest of the space
//! pipeline uses (see `common::models::space::SpaceUser::from_request_parts`).
//! Follow targets stay as the real `User` / `Team` because they're
//! the filter chip's configured target, not a space participant whose
//! identity needs protecting.
//!
//! Best-effort throughout: a deleted source entity falls back to the
//! filter-level labels saved on the report so the row never reads as
//! empty; a deleted user leaves the user fields blank rather than
//! failing the whole page.

use crate::features::spaces::pages::apps::apps::analyzes::*;
use std::collections::{HashMap, HashSet};

pub async fn hydrate_records(
    cli: &aws_sdk_dynamodb::Client,
    space_pk: &Partition,
    filters: &[AnalyzeReportFilter],
    refs: Vec<MatchedRecordRef>,
    anonymous: bool,
) -> Result<Vec<AnalyzeRecordRow>> {
    use crate::common::models::auth::User;
    use crate::common::models::space::SpaceParticipant;
    use crate::common::types::CompositePartition;
    use crate::features::posts::models::Team;
    use crate::features::spaces::pages::actions::actions::discussion::{
        SpacePost, SpacePostComment,
    };
    use crate::features::spaces::pages::actions::actions::poll::SpacePoll;
    use crate::features::spaces::pages::actions::actions::quiz::SpaceQuiz;

    let mut polls: HashMap<String, SpacePoll> = HashMap::new();
    let mut quizzes: HashMap<String, SpaceQuiz> = HashMap::new();
    let mut posts: HashMap<String, SpacePost> = HashMap::new();

    let mut want_poll: HashSet<String> = HashSet::new();
    let mut want_quiz: HashSet<String> = HashSet::new();
    let mut want_post: HashSet<String> = HashSet::new();
    for r in &refs {
        match r.source {
            AnalyzeFilterSource::Poll => {
                want_poll.insert(r.item_id.clone());
            }
            AnalyzeFilterSource::Quiz => {
                want_quiz.insert(r.item_id.clone());
            }
            AnalyzeFilterSource::Discussion => {
                want_post.insert(r.item_id.clone());
            }
            AnalyzeFilterSource::Follow => {}
        }
    }

    for poll_id in want_poll {
        let sk = EntityType::SpacePoll(poll_id.clone());
        if let Ok(Some(p)) = SpacePoll::get(cli, space_pk.clone(), Some(sk)).await {
            polls.insert(poll_id, p);
        }
    }
    for quiz_id in want_quiz {
        let sk = EntityType::SpaceQuiz(quiz_id.clone());
        if let Ok(Some(q)) = SpaceQuiz::get(cli, space_pk.clone(), Some(sk)).await {
            quizzes.insert(quiz_id, q);
        }
    }
    for post_id in want_post {
        let sk = EntityType::SpacePost(post_id.clone());
        if let Ok(Some(p)) = SpacePost::get(cli, space_pk.clone(), Some(sk)).await {
            posts.insert(post_id, p);
        }
    }

    // Actors (the user_pk on each row) need anonymization in anonymous
    // spaces. Follow targets (item_id on Follow rows) are the chip's
    // configured target user — public info — so they always resolve
    // through the global User/Team row regardless of anonymity.
    let mut want_actors: HashSet<String> = HashSet::new();
    let mut want_targets: HashSet<String> = HashSet::new();
    for r in &refs {
        if !r.user_pk.is_empty() {
            want_actors.insert(r.user_pk.clone());
        }
        if r.source == AnalyzeFilterSource::Follow && !r.item_id.is_empty() {
            want_targets.insert(r.item_id.clone());
        }
    }

    // Actor lookup: SpaceParticipant in anonymous spaces, User/Team
    // otherwise. Both fan out via `join_all` so the wall-clock stays
    // bounded by the slowest single GetItem.
    let mut users: HashMap<String, (String, String, String)> = HashMap::new();
    let actor_pks: Vec<(String, Partition)> = want_actors
        .into_iter()
        .filter_map(|s| s.parse::<Partition>().ok().map(|p| (s.clone(), p)))
        .collect();
    if anonymous {
        let lookups = actor_pks.iter().map(|(_key, p)| {
            let p = p.clone();
            let space_pk = space_pk.clone();
            async move {
                SpaceParticipant::get(
                    cli,
                    CompositePartition(space_pk, p),
                    Some(EntityType::SpaceParticipant),
                )
                .await
                .ok()
                .flatten()
                .map(|sp| (sp.username, sp.display_name, sp.profile_url))
            }
        });
        let resolved = futures::future::join_all(lookups).await;
        for ((key, _pk), info) in actor_pks.into_iter().zip(resolved.into_iter()) {
            if let Some(triple) = info {
                users.insert(key, triple);
            }
        }
    } else {
        let lookups = actor_pks.iter().map(|(_key, p)| {
            let p = p.clone();
            async move {
                match &p {
                    Partition::User(_) => User::get(cli, &p, Some(EntityType::User))
                        .await
                        .ok()
                        .flatten()
                        .map(|u| (u.username, u.display_name, u.profile_url)),
                    Partition::Team(_) => Team::get(cli, &p, Some(EntityType::Team))
                        .await
                        .ok()
                        .flatten()
                        .map(|t| (t.username, t.display_name, t.profile_url)),
                    _ => None,
                }
            }
        });
        let resolved = futures::future::join_all(lookups).await;
        for ((key, _pk), info) in actor_pks.into_iter().zip(resolved.into_iter()) {
            if let Some(triple) = info {
                users.insert(key, triple);
            }
        }
    }

    // Follow targets — always real identity.
    let target_pks: Vec<(String, Partition)> = want_targets
        .into_iter()
        .filter_map(|s| s.parse::<Partition>().ok().map(|p| (s.clone(), p)))
        .collect();
    let target_lookups = target_pks.iter().map(|(_key, p)| {
        let p = p.clone();
        async move {
            match &p {
                Partition::User(_) => User::get(cli, &p, Some(EntityType::User))
                    .await
                    .ok()
                    .flatten()
                    .map(|u| (u.username, u.display_name, u.profile_url)),
                Partition::Team(_) => Team::get(cli, &p, Some(EntityType::Team))
                    .await
                    .ok()
                    .flatten()
                    .map(|t| (t.username, t.display_name, t.profile_url)),
                _ => None,
            }
        }
    });
    let target_resolved = futures::future::join_all(target_lookups).await;
    for ((key, _pk), info) in target_pks.into_iter().zip(target_resolved.into_iter()) {
        if let Some(triple) = info {
            users.insert(key, triple);
        }
    }

    let mut rows: Vec<AnalyzeRecordRow> = Vec::with_capacity(refs.len());
    for r in refs {
        let (user_username, user_display_name, user_profile_url) =
            users.get(&r.user_pk).cloned().unwrap_or_default();
        let mut row = AnalyzeRecordRow {
            source: r.source,
            filter_idx: r.filter_idx,
            user_pk: r.user_pk.clone(),
            user_username,
            user_display_name,
            user_profile_url,
            ..Default::default()
        };
        match r.source {
            AnalyzeFilterSource::Poll => {
                if let Some(poll) = polls.get(&r.item_id) {
                    if let Some(question) = poll.questions.get(r.question_idx as usize) {
                        row.question_text = question.title().to_string();
                        row.answer_text = option_text(question, r.option_idx);
                    }
                }
                if row.question_text.is_empty() {
                    if let Some(filter) = filters.get(r.filter_idx as usize) {
                        row.question_text = filter.question_title.clone();
                        row.answer_text = filter.option_text.clone();
                    }
                }
            }
            AnalyzeFilterSource::Quiz => {
                if let Some(quiz) = quizzes.get(&r.item_id) {
                    if let Some(question) = quiz.questions.get(r.question_idx as usize) {
                        row.question_text = question.title().to_string();
                        row.answer_text = option_text(question, r.option_idx);
                    }
                }
                if row.question_text.is_empty() {
                    if let Some(filter) = filters.get(r.filter_idx as usize) {
                        row.question_text = filter.question_title.clone();
                        row.answer_text = filter.option_text.clone();
                    }
                }
            }
            AnalyzeFilterSource::Discussion => {
                if let Some(post) = posts.get(&r.item_id) {
                    row.post_title = post.title.clone();
                }
                if let Ok(comment_sk) = r.sub_id.parse::<EntityType>() {
                    let post_pk = Partition::SpacePost(r.item_id.clone());
                    if let Ok(Some(comment)) =
                        SpacePostComment::get(cli, post_pk, Some(comment_sk)).await
                    {
                        row.comment_text = comment.content;
                        // Override actor info from the comment's
                        // denormalized author ONLY when the space is
                        // not anonymous — otherwise the comment fields
                        // (frozen at write time) could leak the real
                        // identity if the space's anonymity flag was
                        // toggled later. Anonymous mode keeps the
                        // SpaceParticipant lookup already resolved
                        // above.
                        if !anonymous {
                            if !comment.author_username.is_empty() {
                                row.user_username = comment.author_username;
                            }
                            if !comment.author_display_name.is_empty() {
                                row.user_display_name = comment.author_display_name;
                            }
                            if !comment.author_profile_url.is_empty() {
                                row.user_profile_url = comment.author_profile_url;
                            }
                        }
                    }
                }
            }
            AnalyzeFilterSource::Follow => {
                row.target_pk = r.item_id.clone();
                if let Some((u, d, _p)) = users.get(&r.item_id) {
                    row.target_username = u.clone();
                    row.target_display_name = d.clone();
                }
            }
        }
        rows.push(row);
    }

    Ok(rows)
}

fn option_text(
    question: &crate::features::spaces::pages::actions::actions::poll::Question,
    option_idx: u32,
) -> String {
    use crate::features::spaces::pages::actions::actions::poll::Question;
    let idx = option_idx as usize;
    let opts: &[String] = match question {
        Question::SingleChoice(q) => &q.options,
        Question::MultipleChoice(q) => &q.options,
        Question::Checkbox(q) => &q.options,
        Question::Dropdown(q) => &q.options,
        Question::ShortAnswer(_) | Question::Subjective(_) | Question::LinearScale(_) => &[],
    };
    opts.get(idx)
        .cloned()
        .unwrap_or_else(|| format!("Option {}", option_idx + 1))
}
