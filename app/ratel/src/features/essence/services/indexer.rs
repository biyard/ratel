//! Live indexing helpers — called from create/update handlers to mirror
//! every new source entity into an `Essence` row so the user's Essence
//! House stays in sync without a migration roundtrip. Errors are returned
//! but callers typically log-and-swallow, because essence is derived data
//! and must never block the primary create.

use crate::common::*;
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

pub async fn index_poll(
    cli: &aws_sdk_dynamodb::Client,
    poll: &SpacePoll,
    creator_pk: Partition,
) -> Result<()> {
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

pub async fn index_quiz(
    cli: &aws_sdk_dynamodb::Client,
    quiz: &SpaceQuiz,
    creator_pk: Partition,
    action_title: &str,
    action_description: &str,
) -> Result<()> {
    let title = if action_title.trim().is_empty() {
        format!("Quiz {}", strip_prefix(&quiz.sk.to_string()))
    } else {
        action_title.to_string()
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

pub async fn index_discussion_comment(
    cli: &aws_sdk_dynamodb::Client,
    comment: &SpacePostComment,
    space_pk: Partition,
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
        Some(space_pk),
    )
    .await
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
