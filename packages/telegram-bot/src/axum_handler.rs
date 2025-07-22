/*
 FIXME: Refactor this codes.
 a. Separate HTML Templates and Functions.
 b. Create format_time with LocalTimeZone.
*/
use std::sync::Arc;

use chrono::{TimeZone, Utc};
use dto::{
    Result, SprintLeaguePayload, TelegramNotificationPayload, TelegramSubscribe,
    by_axum::axum::{Json, extract::State},
};
use futures::{StreamExt, stream};
use teloxide::{
    Bot,
    payloads::SendMessageSetters,
    prelude::Requester,
    types::{ChatId, InlineKeyboardButton, InlineKeyboardMarkup, ParseMode},
};

use crate::{AppState, config};
use base64::{Engine as _, engine::general_purpose};
use serde::Serialize;

pub async fn notify_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<TelegramNotificationPayload>,
) -> Result<()> {
    let subscribers = match TelegramSubscribe::query_builder()
        .query()
        .map(TelegramSubscribe::from)
        .fetch_all(&state.pool)
        .await
    {
        Ok(subs) => subs,
        Err(e) => {
            tracing::error!("Failed to fetch subscribers: {}", e);
            return Err(e.into());
        }
    };
    let bot = state.bot.clone();
    match payload {
        TelegramNotificationPayload::SprintLeague(sprint_league) => {
            let templates = sprint_league_templates(&sprint_league);
            let keyboards = sprint_league_keyboards(&sprint_league);

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
        }
    }

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

fn sprint_league_templates(payload: &SprintLeaguePayload) -> MessageTemplates {
    let html_template = format!(
        r#"
<b>🏁 {}</b>
<i>{}</i>

<b>⏱️ Period</b>
<code>{}</code> - <code>{}(UTC)</code>

<b>🏃 Participants</b>
- {}
- {}
- {}

    "#,
        payload.title,
        payload.description,
        format_timestamp(payload.started_at),
        format_timestamp(payload.ended_at),
        payload.player_names[0],
        payload.player_names[1],
        payload.player_names[2],
    );

    let html_template_ko = format!(
        r#"
<b>🏁 {}</b>
<i>{}</i>

<b>⏱️ 기간</b>
<code>{}</code> - <code>{}(UTC)</code>

<b>🏃 참여자</b>
- {}
- {}
- {}

        "#,
        payload.title,
        payload.description,
        format_timestamp(payload.started_at),
        format_timestamp(payload.ended_at),
        payload.player_names[0],
        payload.player_names[1],
        payload.player_names[2],
    );

    MessageTemplates {
        html_en: html_template,
        html_ko: html_template_ko,
    }
}
#[derive(Serialize)]
pub struct TgWebParams {
    pub space_id: String,
    pub type_: String,
}
fn sprint_league_keyboards(payload: &SprintLeaguePayload) -> MessageKeyboards {
    let params = TgWebParams {
        space_id: payload.id.to_string(),
        type_: "sprint_league".to_string(),
    };
    let json_string = serde_json::to_string(&params).unwrap();
    let b64_string = general_purpose::STANDARD.encode(json_string);

    let url: dto::reqwest::Url = format!("{}={}", config::get().telegram_mini_app_uri, b64_string,)
        .parse()
        .unwrap();

    let keyboard = InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::url(
        "🔗 Participate Now!".to_string(),
        url.clone(),
    )]]);

    let keyboard_ko = InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::url(
        "🔗 지금 참여하기!".to_string(),
        url,
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
