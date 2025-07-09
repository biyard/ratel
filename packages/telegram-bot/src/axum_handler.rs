use std::sync::Arc;

use chrono::{TimeZone, Utc};
use dto::{
    Result, TelegramNotificationPayload, TelegramSubscribe,
    by_axum::axum::{Json, extract::State},
};
use futures::{StreamExt, stream};
use teloxide::{
    Bot,
    payloads::SendMessageSetters,
    prelude::Requester,
    types::{ChatId, InlineKeyboardButton, InlineKeyboardMarkup, ParseMode},
};

use crate::AppState;

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
    /*
    <b>📊 Voting Status</b>
    <pre>
        {}  : ██████░░░░ 60%
        {}  : ██░░░░░░░░ 20%
        {}  : ░░░░░░░░░░ 0%
    </pre>
       */

    let bot = state.bot.clone();
    let templates = prepare_templates(&payload);
    let keyboards = prepare_keyboards(&payload);

    // 병렬로 메시지 전송 (최대 10개씩 동시 처리)
    let results: Vec<_> = stream::iter(subscribers)
        .map(|sub| {
            let bot = bot.clone();
            let templates = templates.clone();
            let keyboards = keyboards.clone();

            async move { send_message_to_subscriber(bot, sub, templates, keyboards).await }
        })
        .buffer_unordered(10) // 최대 10개 동시 처리
        .collect()
        .await;

    // 결과 통계
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
    let dt = Utc.timestamp_opt(timestamp, 0).unwrap();
    dt.format("%Y-%m-%d %H:%M:%S").to_string()
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

fn prepare_templates(payload: &TelegramNotificationPayload) -> MessageTemplates {
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

👇 <a href="{}"><b>Participate Now!</b></a>
    "#,
        payload.title,
        payload.description,
        format_timestamp(payload.start_at),
        format_timestamp(payload.end_at),
        payload.participants[0],
        payload.participants[1],
        payload.participants[2],
        payload.url,
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

👇 <a href="{}"><b>지금 참여하기!</b></a>
        "#,
        payload.title,
        payload.description,
        format_timestamp(payload.start_at),
        format_timestamp(payload.end_at),
        payload.participants[0],
        payload.participants[1],
        payload.participants[2],
        payload.url,
    );

    MessageTemplates {
        html_en: html_template,
        html_ko: html_template_ko,
    }
}

fn prepare_keyboards(payload: &TelegramNotificationPayload) -> MessageKeyboards {
    let keyboard = InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::url(
        "🔗 Participate Now!".to_string(),
        payload.url.parse().unwrap(),
    )]]);

    let keyboard_ko = InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::url(
        "🔗 지금 참여하기!".to_string(),
        payload.url.parse().unwrap(),
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
