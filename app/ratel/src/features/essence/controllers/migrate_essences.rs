use crate::common::*;
#[cfg(feature = "server")]
use crate::common::models::auth::AdminUser;
#[cfg(feature = "server")]
use crate::features::essence::models::{Essence, UserEssenceStats};
use crate::features::essence::types::*;
use serde::{Deserialize, Serialize};
#[cfg(feature = "server")]
use std::collections::HashMap;

/// Response shape for the admin migration endpoint. Counts per source kind
/// so the operator can spot obvious undercounts.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct MigrateEssencesResponse {
    pub posts_scanned: u32,
    pub spaces_scanned: u32,
    pub discussions_scanned: u32,
    pub posts_migrated: u32,
    pub post_comments_migrated: u32,
    pub discussion_comments_migrated: u32,
    pub polls_migrated: u32,
    pub quizzes_migrated: u32,
    /// Number of `UserEssenceStats` rows rebuilt from scratch. Counter drift
    /// from earlier writes (bumps that silently failed) is reconciled here,
    /// and the new per-kind counters are populated for users created before
    /// they existed on the model.
    pub stats_rebuilt: u32,
    /// Non-fatal errors during the run — logged server-side, returned here
    /// as a count so the operator can spot trouble without tailing logs.
    pub errors: u32,
}

/// Admin-only backfill. Scans every source entity type directly via the
/// `type-index` GSI (for unit-sk entities) or by iterating parent partitions
/// (for nested entities), and idempotently upserts an Essence row per
/// source. Attribution (`user_pk` on the Essence row) is copied from each
/// source row's own author field, so team-authored content stays attributed
/// to the team and user-authored content to the user.
#[post("/api/admin/essences/migrate", _user: AdminUser)]
pub async fn migrate_essences_handler() -> Result<MigrateEssencesResponse> {
    let conf = crate::common::config::ServerConfig::default();
    let cli = conf.dynamodb();

    let mut out = MigrateEssencesResponse::default();

    migrate_posts_and_comments(cli, &mut out).await;
    migrate_spaces_content(cli, &mut out).await;
    rebuild_user_essence_stats(cli, &mut out).await;

    Ok(out)
}

/// Walk every `Essence` row, group by `user_pk`, and overwrite each user's
/// `UserEssenceStats` with the recomputed totals. This reconciles any
/// drift from prior `bump_stats` calls that failed silently (the bump path
/// intentionally logs-and-continues so a stats hiccup never fails a user
/// write). It also populates the new `total_{kind}` fields for users who
/// predate them.
#[cfg(feature = "server")]
async fn rebuild_user_essence_stats(
    cli: &aws_sdk_dynamodb::Client,
    out: &mut MigrateEssencesResponse,
) {
    let essences = match Essence::find_all(cli, EntityType::Essence(String::new()), Essence::opt_all()).await {
        Ok((v, _)) => v,
        Err(e) => {
            crate::error!("migrate: scan essences for stats rebuild failed: {e}");
            out.errors += 1;
            return;
        }
    };

    let mut totals_by_user: HashMap<String, UserEssenceStats> = HashMap::new();
    for row in essences {
        let key = row.pk.to_string();
        let entry = totals_by_user
            .entry(key)
            .or_insert_with(UserEssenceStats::default);
        entry.total_sources += 1;
        entry.total_words += row.word_count;
        match row.source_kind {
            EssenceSourceKind::Notion => entry.total_notion += 1,
            EssenceSourceKind::Post => entry.total_post += 1,
            EssenceSourceKind::PostComment | EssenceSourceKind::DiscussionComment => {
                entry.total_comment += 1
            }
            EssenceSourceKind::Poll => entry.total_poll += 1,
            EssenceSourceKind::Quiz => entry.total_quiz += 1,
        }
    }

    for (user_pk_str, totals) in totals_by_user {
        let user_pk: Partition = match user_pk_str.parse() {
            Ok(p) => p,
            Err(e) => {
                crate::error!("migrate: skip stats rebuild — bad user pk {user_pk_str}: {e}");
                out.errors += 1;
                continue;
            }
        };
        if let Err(e) = Essence::replace_stats(cli, &user_pk, totals).await {
            crate::error!("migrate: replace stats failed for {user_pk_str}: {e}");
            out.errors += 1;
            continue;
        }
        out.stats_rebuilt += 1;
    }
}

#[cfg(feature = "server")]
async fn migrate_posts_and_comments(
    cli: &aws_sdk_dynamodb::Client,
    out: &mut MigrateEssencesResponse,
) {
    use crate::features::posts::models::{Post, PostComment};

    let (posts, _) = match Post::find_all(cli, EntityType::Post, Post::opt_all()).await {
        Ok(v) => v,
        Err(e) => {
            crate::error!("migrate: list posts failed: {e}");
            out.errors += 1;
            return;
        }
    };
    out.posts_scanned = posts.len() as u32;

    let comment_sk_prefix = EntityType::PostComment(String::new()).to_string();

    for post in posts {
        let title = if post.title.is_empty() {
            "(Untitled post)".to_string()
        } else {
            post.title.clone()
        };
        let source_path = format!("Ratel post · {}", strip_prefix(&post.pk.to_string()));
        let word_count = (estimate_word_count(&post.html_contents)
            + post.title.split_whitespace().count() as u32) as i64;

        if let Err(e) = Essence::upsert_for_source(
            cli,
            post.user_pk.clone(),
            post.pk.clone(),
            EntityType::Post,
            EssenceSourceKind::Post,
            title,
            source_path,
            word_count,
            None,
        )
        .await
        {
            crate::error!("migrate: upsert post essence failed: {e}");
            out.errors += 1;
        } else {
            out.posts_migrated += 1;
        }

        let opts = PostComment::opt_all().sk(comment_sk_prefix.clone());
        let (comments, _) = match PostComment::query(cli, post.pk.clone(), opts).await {
            Ok(v) => v,
            Err(e) => {
                crate::error!("migrate: list post comments failed: {e}");
                out.errors += 1;
                continue;
            }
        };

        for c in comments {
            let title = summarize(&c.content, 80);
            let source_path = format!(
                "Ratel comment · {} / {}",
                strip_prefix(&c.pk.to_string()),
                strip_prefix(&c.sk.to_string())
            );
            let word_count = c.content.split_whitespace().count() as i64;

            if let Err(e) = Essence::upsert_for_source(
                cli,
                c.author_pk.clone(),
                c.pk.clone(),
                c.sk.clone(),
                EssenceSourceKind::PostComment,
                title,
                source_path,
                word_count,
                None,
            )
            .await
            {
                crate::error!("migrate: upsert post comment essence failed: {e}");
                out.errors += 1;
                continue;
            }
            out.post_comments_migrated += 1;
        }
    }
}

#[cfg(feature = "server")]
async fn migrate_spaces_content(
    cli: &aws_sdk_dynamodb::Client,
    out: &mut MigrateEssencesResponse,
) {
    use crate::common::models::space::SpaceCommon;
    use crate::features::spaces::pages::actions::actions::discussion::{
        SpacePost, SpacePostComment,
    };
    use crate::features::spaces::pages::actions::actions::poll::SpacePoll;
    use crate::features::spaces::pages::actions::actions::quiz::SpaceQuiz;

    let (spaces, _) = match SpaceCommon::find_all(
        cli,
        EntityType::SpaceCommon,
        SpaceCommon::opt_all(),
    )
    .await
    {
        Ok(v) => v,
        Err(e) => {
            crate::error!("migrate: list spaces failed: {e}");
            out.errors += 1;
            return;
        }
    };
    out.spaces_scanned = spaces.len() as u32;

    let poll_sk_prefix = EntityType::SpacePoll(String::new()).to_string();
    let quiz_sk_prefix = EntityType::SpaceQuiz(String::new()).to_string();
    let discussion_sk_prefix = EntityType::SpacePost(String::new()).to_string();
    let discussion_comment_sk_prefix = EntityType::SpacePostComment(String::new()).to_string();

    for space in spaces {
        let space_id_str = match &space.pk {
            Partition::Space(id) => id.clone(),
            _ => continue,
        };
        let space_pk = space.pk.clone();
        let space_sub = SpacePartition(space_id_str);
        let creator_pk = space.user_pk.clone();

        let opts = SpacePoll::opt_all().sk(poll_sk_prefix.clone());
        match SpacePoll::query(cli, space_pk.clone(), opts).await {
            Ok((polls, _)) => {
                for poll in polls {
                    let title = if poll.title.is_empty() {
                        "(Untitled poll)".to_string()
                    } else {
                        poll.title.clone()
                    };
                    let source_path = format!(
                        "Ratel poll · {} / {}",
                        strip_prefix(&poll.pk.to_string()),
                        strip_prefix(&poll.sk.to_string())
                    );
                    let word_count = (poll.title.split_whitespace().count()
                        + poll.description.split_whitespace().count())
                        as i64;

                    if let Err(e) = Essence::upsert_for_source(
                        cli,
                        creator_pk.clone(),
                        poll.pk.clone(),
                        poll.sk.clone(),
                        EssenceSourceKind::Poll,
                        title,
                        source_path,
                        word_count,
                        Some(space_pk.clone()),
                    )
                    .await
                    {
                        crate::error!("migrate: upsert poll essence failed: {e}");
                        out.errors += 1;
                        continue;
                    }
                    out.polls_migrated += 1;
                }
            }
            Err(e) => {
                crate::error!("migrate: list polls for space failed: {e}");
                out.errors += 1;
            }
        }

        let opts = SpaceQuiz::opt_all().sk(quiz_sk_prefix.clone());
        match SpaceQuiz::query(cli, space_pk.clone(), opts).await {
            Ok((quizzes, _)) => {
                use crate::features::spaces::pages::actions::models::SpaceAction;

                for quiz in quizzes {
                    let action_key =
                        CompositePartition(space_sub.clone(), quiz.sk.to_string());
                    let (action_title, action_description) =
                        match SpaceAction::get(cli, &action_key, Some(EntityType::SpaceAction))
                            .await
                        {
                            Ok(Some(action)) => (action.title, action.description),
                            _ => (String::new(), String::new()),
                        };

                    if let Err(e) = crate::features::essence::services::index_quiz(
                        cli,
                        &quiz,
                        creator_pk.clone(),
                        &action_title,
                        &action_description,
                    )
                    .await
                    {
                        crate::error!("migrate: upsert quiz essence failed: {e}");
                        out.errors += 1;
                        continue;
                    }
                    out.quizzes_migrated += 1;
                }
            }
            Err(e) => {
                crate::error!("migrate: list quizzes for space failed: {e}");
                out.errors += 1;
            }
        }

        let opts = SpacePost::opt_all().sk(discussion_sk_prefix.clone());
        let discussions = match SpacePost::query(cli, space_pk.clone(), opts).await {
            Ok((v, _)) => v,
            Err(e) => {
                crate::error!("migrate: list discussions for space failed: {e}");
                out.errors += 1;
                continue;
            }
        };
        out.discussions_scanned += discussions.len() as u32;

        for discussion in discussions {
            let discussion_id = match &discussion.sk {
                EntityType::SpacePost(id) => id.clone(),
                _ => continue,
            };
            let discussion_pk = Partition::SpacePost(discussion_id);

            let opts = SpacePostComment::opt_all().sk(discussion_comment_sk_prefix.clone());
            match SpacePostComment::query(cli, discussion_pk, opts).await {
                Ok((comments, _)) => {
                    for c in comments {
                        let title = summarize(&c.content, 80);
                        let source_path = format!(
                            "Discussion · {} / {}",
                            strip_prefix(&c.pk.to_string()),
                            strip_prefix(&c.sk.to_string())
                        );
                        let word_count = c.content.split_whitespace().count() as i64;

                        if let Err(e) = Essence::upsert_for_source(
                            cli,
                            c.author_pk.clone(),
                            c.pk.clone(),
                            c.sk.clone(),
                            EssenceSourceKind::DiscussionComment,
                            title,
                            source_path,
                            word_count,
                            Some(space_pk.clone()),
                        )
                        .await
                        {
                            crate::error!("migrate: upsert discussion comment essence failed: {e}");
                            out.errors += 1;
                            continue;
                        }
                        out.discussion_comments_migrated += 1;
                    }
                }
                Err(e) => {
                    crate::error!("migrate: list discussion comments failed: {e}");
                    out.errors += 1;
                }
            }
        }
    }
}

/// Drop the `PREFIX#` that `DynamoEnum` Display emits, returning just the
/// id portion for display. Idempotent on strings that have no `#`.
#[cfg(feature = "server")]
fn strip_prefix(raw: &str) -> &str {
    raw.split_once('#').map(|(_, tail)| tail).unwrap_or(raw)
}

/// Strip HTML tags and return a rough word count — matches how the running
/// create_post handler estimates body length so migrated rows are
/// consistent with newly-created ones.
#[cfg(feature = "server")]
fn estimate_word_count(html: &str) -> u32 {
    let re_tags = regex::Regex::new(r"<[^>]+>").unwrap();
    let text = re_tags.replace_all(html, " ");
    text.split_whitespace().count() as u32
}

/// First `n` characters of `s`, stopping at a word boundary. Used to build
/// a row title from comment content that is stored as free-form text.
#[cfg(feature = "server")]
fn summarize(s: &str, n: usize) -> String {
    let trimmed = s.trim();
    if trimmed.chars().count() <= n {
        return trimmed.to_string();
    }
    let mut out: String = trimmed.chars().take(n).collect();
    out.push('…');
    out
}
