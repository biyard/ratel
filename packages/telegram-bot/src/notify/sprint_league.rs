use chrono::{TimeZone, Utc};
use futures::{StreamExt, stream};

use dto::{Result, Space, SprintLeague, TelegramSubscribe, sqlx::PgPool};
use teloxide::{
    Bot,
    payloads::SendMessageSetters,
    prelude::Requester,
    types::{ChatId, InlineKeyboardButton, InlineKeyboardMarkup, ParseMode},
};

use crate::{TgWebCommand, generate_link};

pub async fn handler(pool: &PgPool, bot: &Bot, space_id: i64) -> Result<()> {
    let space = Space::query_builder(0)
        .id_equals(space_id)
        .sprint_leagues_builder(SprintLeague::query_builder(0))
        .query()
        .map(Space::from)
        .fetch_one(pool)
        .await?;

    let sprint_league = space.sprint_leagues.get(0);
    if sprint_league.is_none() {
        tracing::warn!("No sprint league found for space {}", space_id);
        return Ok(());
    }
    let sprint_league = sprint_league.unwrap();
    tracing::debug!("{:?}", sprint_league);
    let subscribers = match TelegramSubscribe::query_builder()
        .query()
        .map(TelegramSubscribe::from)
        .fetch_all(pool)
        .await
    {
        Ok(subs) => subs,
        Err(e) => {
            tracing::error!("Failed to fetch subscribers: {}", e);
            return Err(e.into());
        }
    };

    //FIXME: FILTER SUBSCRIBERS BY SPACE PERMISSIONS

    let title = &space.title.unwrap_or("Sprint League".to_string());
    let templates = sprint_league_templates(
        title,
        space.started_at.unwrap_or_default(),
        space.ended_at.unwrap_or_default(),
        sprint_league
            .players
            .iter()
            .map(|p| p.name.as_str())
            .collect::<Vec<&str>>(),
    );
    let keyboards = sprint_league_keyboards(space_id);

    let results: Vec<_> = stream::iter(subscribers)
        .map(|sub| {
            let bot = bot.clone();
            let templates = templates.clone();
            let keyboards = keyboards.clone();

            async move { send_message_to_subscriber(bot, sub, templates, keyboards).await }
        })
        .buffer_unordered(10)
        .collect()
        .await;

    let (success_count, error_count) =
        results
            .iter()
            .fold((0, 0), |(success, error), result| match result {
                Ok(_) => (success + 1, error),
                Err(_) => (success, error + 1),
            });

    tracing::info!(
        "Message sending completed: {} success, {} errors",
        success_count,
        error_count
    );

    Ok(())
}

fn format_timestamp(timestamp: i64) -> String {
    match Utc.timestamp_opt(timestamp, 0).single() {
        Some(dt) => dt.format("%Y-%m-%d %H:%M:%S").to_string(),
        None => "Invalid timestamp".to_string(),
    }
}

#[derive(Clone)]
struct MessageTemplates {
    html_en: String,
    html_ko: String,
}

#[derive(Clone)]
struct MessageKeyboards {
    keyboard_en: InlineKeyboardMarkup,
    keyboard_ko: InlineKeyboardMarkup,
}

fn sprint_league_templates(
    title: &str,
    started_at: i64,
    ended_at: i64,
    player_names: Vec<&str>,
) -> MessageTemplates {
    let html_template = format!(
        r#"
<b>🏁 {}</b>

<b>⏱️ Period</b>
<code>{}</code> - <code>{}(UTC)</code>

<b>🏃 Participants</b>
- {}
- {}
- {}

    "#,
        title,
        format_timestamp(started_at),
        format_timestamp(ended_at),
        player_names[0],
        player_names[1],
        player_names[2],
    );

    let html_template_ko = format!(
        r#"
<b>🏁 {}</b>


<b>⏱️ 기간</b>
<code>{}</code> - <code>{}(UTC)</code>

<b>🏃 참여자</b>
- {}
- {}
- {}

        "#,
        title,
        format_timestamp(started_at),
        format_timestamp(ended_at),
        player_names[0],
        player_names[1],
        player_names[2],
    );

    MessageTemplates {
        html_en: html_template,
        html_ko: html_template_ko,
    }
}

fn sprint_league_keyboards(space_id: i64) -> MessageKeyboards {
    let link = generate_link(TgWebCommand::SprintLeague { space_id });

    let keyboard = InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::url(
        "🔗 Participate Now!".to_string(),
        link.parse().unwrap(),
    )]]);

    let keyboard_ko = InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::url(
        "🔗 지금 참여하기!".to_string(),
        link.parse().unwrap(),
    )]]);

    MessageKeyboards {
        keyboard_en: keyboard,
        keyboard_ko: keyboard_ko,
    }
}

async fn send_message_to_subscriber(
    bot: Bot,
    sub: TelegramSubscribe,
    templates: MessageTemplates,
    keyboards: MessageKeyboards,
) -> Result<()> {
    let chat_id = ChatId(sub.chat_id);
    let (html, keyboard) = match sub.lang.as_deref() {
        Some("ko") => (templates.html_ko, keyboards.keyboard_ko),
        _ => (templates.html_en, keyboards.keyboard_en),
    };

    match bot
        .send_message(chat_id, &html)
        .reply_markup(keyboard)
        .parse_mode(ParseMode::Html)
        .await
    {
        Ok(_) => {
            tracing::debug!("Successfully sent message to {}", sub.chat_id);
            Ok(())
        }
        Err(e) => {
            tracing::error!("Failed to send message to {}: {}", sub.chat_id, e);
            Err(e.into())
        }
    }
}
