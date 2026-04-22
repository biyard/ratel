//! Live indexing helpers — invoked by the DynamoDB Stream pipeline (and the
//! migrate endpoint) to mirror every source entity into an `Essence` row so
//! the user's Essence House stays in sync without a manual roundtrip.
//!
//! Every function is intentionally `(cli, &entity)` shaped so the stream
//! dispatcher can call them uniformly after deserializing the new image,
//! without needing extra context from the original write site. Lookups for
//! related entities (creator id, action title, etc.) happen inline.

use crate::common::*;
use crate::common::models::space::SpaceCommon;
use crate::features::essence::models::Essence;
use crate::features::essence::types::*;
use crate::features::posts::models::{Post, PostComment};
use crate::features::spaces::pages::actions::actions::discussion::SpacePostComment;
use crate::features::spaces::pages::actions::actions::poll::SpacePoll;
use crate::features::spaces::pages::actions::actions::quiz::SpaceQuiz;

pub async fn index_post(cli: &aws_sdk_dynamodb::Client, post: &Post) -> Result<()> {
    let title = if post.title.is_empty() {
        "(Untitled post)".to_string()
    } else {
        post.title.clone()
    };
    let source_path = format!("Ratel post · {}", strip_prefix(&post.pk.to_string()));
    let word_count = (estimate_word_count(&post.html_contents)
        + post.title.split_whitespace().count() as u32) as i64;

    Essence::upsert_for_source(
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
}

pub async fn detach_post(cli: &aws_sdk_dynamodb::Client, post: &Post) -> Result<()> {
    Essence::delete_for_source(cli, post.user_pk.clone(), &post.pk, &EntityType::Post).await
}

pub async fn index_post_comment(
    cli: &aws_sdk_dynamodb::Client,
    comment: &PostComment,
) -> Result<()> {
    let title = summarize(&comment.content, 80);
    let source_path = format!(
        "Ratel comment · {} / {}",
        strip_prefix(&comment.pk.to_string()),
        strip_prefix(&comment.sk.to_string())
    );
    let word_count = comment.content.split_whitespace().count() as i64;

    Essence::upsert_for_source(
        cli,
        comment.author_pk.clone(),
        comment.pk.clone(),
        comment.sk.clone(),
        EssenceSourceKind::PostComment,
        title,
        source_path,
        word_count,
        None,
    )
    .await
}

pub async fn detach_post_comment(
    cli: &aws_sdk_dynamodb::Client,
    comment: &PostComment,
) -> Result<()> {
    Essence::delete_for_source(cli, comment.author_pk.clone(), &comment.pk, &comment.sk).await
}

/// Looks up the parent space's `user_pk` to attribute the essence row to the
/// space creator (polls/quizzes are creator-authored content, so attribution
/// follows the space owner — not whoever called `update_poll`).
pub async fn index_poll(cli: &aws_sdk_dynamodb::Client, poll: &SpacePoll) -> Result<()> {
    let creator_pk = lookup_space_creator(cli, &poll.pk).await?;
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
        + poll.description.split_whitespace().count()) as i64;

    Essence::upsert_for_source(
        cli,
        creator_pk,
        poll.pk.clone(),
        poll.sk.clone(),
        EssenceSourceKind::Poll,
        title,
        source_path,
        word_count,
        Some(poll.pk.clone()),
    )
    .await
}

pub async fn detach_poll(cli: &aws_sdk_dynamodb::Client, poll: &SpacePoll) -> Result<()> {
    let creator_pk = lookup_space_creator(cli, &poll.pk).await?;
    Essence::delete_for_source(cli, creator_pk, &poll.pk, &poll.sk).await
}

/// Quizzes carry their displayable title/description on the `SpaceAction`
/// (the one-per-action metadata row keyed by `(space, action_id)`). When
/// indexing, we look that row up to mirror the same text the user sees in
/// the arena card.
pub async fn index_quiz(cli: &aws_sdk_dynamodb::Client, quiz: &SpaceQuiz) -> Result<()> {
    use crate::features::spaces::pages::actions::models::SpaceAction;

    let creator_pk = lookup_space_creator(cli, &quiz.pk).await?;

    let space_id_str = match &quiz.pk {
        Partition::Space(id) => id.clone(),
        _ => return Err(EssenceError::ReadFailed.into()),
    };
    // `SpaceAction.pk.1` stores the quiz id as a BARE uuid (no `SPACE_QUIZ#`
    // prefix), because `create_quiz.rs` passes
    // `SpaceQuizEntityType::from(quiz.sk).to_string()` when building the
    // action row — and `SpaceQuizEntityType`'s `Display` strips the prefix
    // via the `SubPartition` macro. So we must extract the bare id from
    // `quiz.sk` here; using `quiz.sk.to_string()` directly would look up
    // `"SPACE_QUIZ#uuid"` and miss the real row (which is why the legacy
    // migrate code silently returned empty title/description).
    let quiz_id = match &quiz.sk {
        EntityType::SpaceQuiz(id) => id.clone(),
        _ => return Err(EssenceError::ReadFailed.into()),
    };
    let action_key = CompositePartition(SpacePartition(space_id_str), quiz_id);
    let (action_title, action_description) =
        match SpaceAction::get(cli, &action_key, Some(EntityType::SpaceAction)).await {
            Ok(Some(action)) => (action.title, action.description),
            _ => (String::new(), String::new()),
        };

    let title = if action_title.trim().is_empty() {
        format!("Quiz {}", strip_prefix(&quiz.sk.to_string()))
    } else {
        action_title.clone()
    };
    let source_path = format!(
        "Ratel quiz · {} / {}",
        strip_prefix(&quiz.pk.to_string()),
        strip_prefix(&quiz.sk.to_string())
    );
    let word_count = (action_title.split_whitespace().count()
        + action_description.split_whitespace().count()) as i64;

    Essence::upsert_for_source(
        cli,
        creator_pk,
        quiz.pk.clone(),
        quiz.sk.clone(),
        EssenceSourceKind::Quiz,
        title,
        source_path,
        word_count,
        Some(quiz.pk.clone()),
    )
    .await
}

pub async fn detach_quiz(cli: &aws_sdk_dynamodb::Client, quiz: &SpaceQuiz) -> Result<()> {
    let creator_pk = lookup_space_creator(cli, &quiz.pk).await?;
    Essence::delete_for_source(cli, creator_pk, &quiz.pk, &quiz.sk).await
}

/// `space_pk` for the essence row comes from the comment's denormalized
/// `space_pk` field (set by `SpacePostComment::new`). Older rows missing the
/// field index without `space_pk`, which only degrades the "open in space"
/// shortcut and not list/search.
pub async fn index_discussion_comment(
    cli: &aws_sdk_dynamodb::Client,
    comment: &SpacePostComment,
) -> Result<()> {
    let title = summarize(&comment.content, 80);
    let source_path = format!(
        "Discussion · {} / {}",
        strip_prefix(&comment.pk.to_string()),
        strip_prefix(&comment.sk.to_string())
    );
    let word_count = comment.content.split_whitespace().count() as i64;

    Essence::upsert_for_source(
        cli,
        comment.author_pk.clone(),
        comment.pk.clone(),
        comment.sk.clone(),
        EssenceSourceKind::DiscussionComment,
        title,
        source_path,
        word_count,
        comment.space_pk.clone(),
    )
    .await
}

pub async fn detach_discussion_comment(
    cli: &aws_sdk_dynamodb::Client,
    comment: &SpacePostComment,
) -> Result<()> {
    Essence::delete_for_source(cli, comment.author_pk.clone(), &comment.pk, &comment.sk).await
}

/// Resolve `space.user_pk` from a `Space` partition. Used to attribute
/// poll/quiz essence rows to the space creator.
async fn lookup_space_creator(
    cli: &aws_sdk_dynamodb::Client,
    space_pk: &Partition,
) -> Result<Partition> {
    let space = SpaceCommon::get(cli, space_pk, Some(&EntityType::SpaceCommon))
        .await
        .map_err(|e| {
            crate::error!("essence indexer: space lookup failed: {e}");
            EssenceError::ReadFailed
        })?
        .ok_or_else(|| {
            crate::error!("essence indexer: space {space_pk} not found");
            EssenceError::ReadFailed
        })?;
    Ok(space.user_pk)
}

/// Drop the `PREFIX#` that `DynamoEnum` Display emits, returning just the
/// id portion for display. Idempotent on strings that have no `#`.
fn strip_prefix(raw: &str) -> &str {
    raw.split_once('#').map(|(_, tail)| tail).unwrap_or(raw)
}

/// Strip HTML tags and return a rough word count — matches how the running
/// create_post handler estimates body length so migrated rows are
/// consistent with newly-created ones.
fn estimate_word_count(html: &str) -> u32 {
    let re_tags = regex::Regex::new(r"<[^>]+>").unwrap();
    let text = re_tags.replace_all(html, " ");
    text.split_whitespace().count() as u32
}

/// First `n` characters of `s`, stopping at a word boundary. Used to build
/// a row title from comment content that is stored as free-form text.
fn summarize(s: &str, n: usize) -> String {
    let trimmed = s.trim();
    if trimmed.chars().count() <= n {
        return trimmed.to_string();
    }
    let mut out: String = trimmed.chars().take(n).collect();
    out.push('…');
    out
}
