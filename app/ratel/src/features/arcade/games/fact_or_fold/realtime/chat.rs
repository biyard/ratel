//! `RoomChannel` implementation for FOF chat.
//!
//! Channel kind: `"fof.chat"`. ChannelId form: `fof.chat:{round_id}`.
//!
//! ### Authorize hook
//!
//! Only round participants may subscribe (gated against
//! `FactFoldRound.participant_pks`). Initial state = the latest N
//! chat rows already in the DDB transcript, so the client renders a
//! useful backlog on first connect / reconnect rather than starting
//! empty.
//!
//! ### Fan-out
//!
//! v1 doesn't publish from inside this handler. The
//! `POST /rounds/{id}/chat` controller only writes a
//! `FactFoldChatMessage` row; the DDB Stream listener
//! ([`crate::common::stream_handler`] for local-dev,
//! `EventBridgeEnvelope::proc` for Lambda) detects the new row and
//! calls `global_hub().publish(...)` so every SSE invocation in
//! the cluster fans it out to its own subscribers.

use crate::common::*;
use crate::features::arcade::ArcadeError;
use crate::features::arcade::games::fact_or_fold::models::{FactFoldChatMessage, FactFoldRound};
use crate::features::arcade::realtime::channel::{ChannelContext, ChannelId, RoomChannel};
use async_trait::async_trait;

/// How many recent chat messages to surface on the subscribe
/// snapshot. Caps reconnect catch-up cost.
const CHAT_HISTORY_LIMIT: usize = 50;

/// Wire-format chat row sent over SSE (and in the initial snapshot).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatMessagePayload {
    pub msg_id: String,
    pub author_pk: String,
    pub text: String,
    pub sent_at: i64,
}

impl From<FactFoldChatMessage> for ChatMessagePayload {
    fn from(row: FactFoldChatMessage) -> Self {
        Self {
            msg_id: row.id().unwrap_or_default(),
            author_pk: row.author_pk.to_string(),
            text: row.text,
            sent_at: row.sent_at,
        }
    }
}

pub struct FactFoldChatChannel {
    cli: aws_sdk_dynamodb::Client,
}

impl FactFoldChatChannel {
    pub fn new(cli: aws_sdk_dynamodb::Client) -> Self {
        Self { cli }
    }
}

#[async_trait]
impl RoomChannel for FactFoldChatChannel {
    fn kind(&self) -> &'static str {
        "fof.chat"
    }

    async fn authorize(
        &self,
        ctx: &ChannelContext,
        channel: &ChannelId,
        _params: serde_json::Value,
    ) -> crate::common::Result<serde_json::Value> {
        let round_id = channel.inner().to_string();
        if round_id.is_empty() {
            return Err(ArcadeError::ChannelPayloadInvalid.into());
        }

        // Participant gate: the caller's USER#{user_id} pk must be
        // in the round's participant list. We never expose the
        // round's pk shape to the channel layer — the round read
        // happens here and the channel layer only sees the result.
        let (pk, sk) = FactFoldRound::keys(&round_id);
        let round = FactFoldRound::get(&self.cli, &pk, Some(sk))
            .await
            .map_err(|e| {
                crate::error!("fof.chat authorize round read failed: {e}");
                ArcadeError::StorageFailure
            })?
            .ok_or(ArcadeError::ChannelForbidden)?;

        let user_pk_str = format!("USER#{}", ctx.user_id);
        let is_participant = round
            .participant_pks
            .iter()
            .any(|p| p.to_string() == user_pk_str);
        if !is_participant {
            return Err(ArcadeError::ChannelForbidden.into());
        }

        // Backlog: load the latest CHAT_HISTORY_LIMIT messages for
        // this round. v1 simple sk-prefix scan; pagination lands
        // when round transcripts grow past a single page (rare in
        // practice — 4 players × 70s debate × 80 char cap).
        let opts = FactFoldChatMessage::opt()
            .sk("FACT_FOLD_CHAT".to_string())
            .limit(CHAT_HISTORY_LIMIT as i32);
        let (rows, _) = FactFoldChatMessage::query(&self.cli, pk, opts)
            .await
            .map_err(|e| {
                crate::error!("fof.chat history query failed: {e}");
                ArcadeError::StorageFailure
            })?;

        let payloads: Vec<ChatMessagePayload> = rows.into_iter().map(Into::into).collect();
        Ok(serde_json::json!({ "history": payloads }))
    }
}
