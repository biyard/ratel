//! Lobby + matching endpoints for *Fact or Fold*.
//!
//! Surface (PR3):
//!   GET   /api/fact-or-fold/lobby
//!   POST  /api/fact-or-fold/lobby/join
//!   POST  /api/fact-or-fold/lobby/leave
//!   GET   /api/fact-or-fold/rounds/{round_id}
//!
//! Matching algorithm:
//!   1. Read FactFoldLobby singleton.
//!   2. If no waiting round → create one (pick the next eligible
//!      headline; transition it to Live).
//!   3. Add the user to Round.participant_pks (no-op if already in,
//!      reject if full or below min_bet_rp).
//!   4. If after the join the round is full → flip status to
//!      NewsReveal, set started_at, clear the lobby pointer.
//!
//! Concurrency: single app-shell instance (MVP, per design doc
//! §Realtime channel). Two simultaneous joins racing on the same
//! pointer would be possible in a multi-instance deployment;
//! conditional updates land alongside multi-instance work.

use crate::common::*;
use crate::features::fact_or_fold::types::*;

#[cfg(feature = "server")]
use crate::common::models::auth::User;
#[cfg(feature = "server")]
use crate::features::fact_or_fold::models::{
    FactFoldHeadline, FactFoldLobby, FactFoldRound, FactFoldSettings,
};

#[cfg(feature = "server")]
const HEADLINE_SK_PREFIX: &str = "FACT_FOLD_HEADLINE";

// ── Internal helpers ──────────────────────────────────────────────

#[cfg(feature = "server")]
async fn load_settings_or_default(cli: &aws_sdk_dynamodb::Client) -> FactOrFoldSettingsResponse {
    FactFoldSettings::get_or_default(cli)
        .await
        .unwrap_or_default()
}

#[cfg(feature = "server")]
async fn pick_next_headline(
    cli: &aws_sdk_dynamodb::Client,
) -> crate::common::Result<Option<FactFoldHeadline>> {
    // List from the anchor pk. v1 is small enough that a single page
    // (limit 200) is fine; pagination lands when the lifetime queue
    // exceeds it.
    let opts = FactFoldHeadline::opt()
        .sk(HEADLINE_SK_PREFIX.to_string())
        .limit(200);
    let (rows, _) = FactFoldHeadline::query(cli, FactFoldHeadline::anchor_pk(), opts)
        .await
        .map_err(|e| {
            crate::error!("pick_next_headline query failed: {e}");
            FactOrFoldError::StorageFailure
        })?;

    let now = crate::common::utils::time::get_now_timestamp_millis();

    // Eligibility: Live (already activated, but not yet bound to a
    // round) OR Scheduled with scheduled_at <= now.
    let mut eligible: Vec<FactFoldHeadline> = rows
        .into_iter()
        .filter(|h| match h.status {
            HeadlineStatus::Live => true,
            HeadlineStatus::Scheduled => h.scheduled_at.map(|ts| ts <= now).unwrap_or(false),
            _ => false,
        })
        .collect();

    // Pick the oldest scheduled time first so the queue is FIFO.
    eligible.sort_by(|a, b| {
        a.scheduled_at
            .unwrap_or(a.created_at)
            .cmp(&b.scheduled_at.unwrap_or(b.created_at))
    });

    Ok(eligible.into_iter().next())
}

#[cfg(feature = "server")]
async fn upsert_lobby_pointer(
    cli: &aws_sdk_dynamodb::Client,
    current_round_id: Option<String>,
) -> crate::common::Result<()> {
    let mut row = FactFoldLobby::get_or_default(cli).await?;
    let now = crate::common::utils::time::get_now_timestamp_millis();
    row.current_round_id = current_round_id;
    row.updated_at = now;
    if row.created_at == 0 {
        row.created_at = now;
    }
    row.upsert(cli).await.map_err(|e| {
        crate::error!("upsert_lobby_pointer failed: {e}");
        FactOrFoldError::StorageFailure.into()
    })
}

#[cfg(feature = "server")]
fn round_to_response(row: &FactFoldRound) -> RoundResponse {
    let id = row.id().unwrap_or_default();
    RoundResponse {
        id: FactFoldRoundEntityType(id),
        headline_id: FactFoldHeadlineEntityType(row.headline_id.clone()),
        status: row.status,
        participant_pks: row
            .participant_pks
            .iter()
            .map(|p| p.to_string())
            .collect(),
        started_at: row.started_at,
        settled_at: row.settled_at,
        created_at: row.created_at,
        updated_at: row.updated_at,
    }
}

// ── GET /api/fact-or-fold/lobby ───────────────────────────────────

#[get("/api/fact-or-fold/lobby", user: User)]
pub async fn get_lobby_handler() -> Result<LobbyResponse> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let settings = load_settings_or_default(cli).await;
    let lobby = FactFoldLobby::get_or_default(cli).await.map_err(|e| {
        crate::error!("get_lobby_handler lobby read failed: {e}");
        FactOrFoldError::StorageFailure
    })?;

    let user_pk_str = user.pk.to_string();

    let mut current_round: Option<RoundResponse> = None;
    let mut already_joined = false;

    if let Some(round_id) = lobby.current_round_id.as_deref() {
        let (pk, sk) = FactFoldRound::keys(round_id);
        if let Some(round) = FactFoldRound::get(cli, &pk, Some(sk)).await.map_err(|e| {
            crate::error!("get_lobby_handler round read failed: {e}");
            FactOrFoldError::StorageFailure
        })? {
            already_joined = round
                .participant_pks
                .iter()
                .any(|p| p.to_string() == user_pk_str);
            current_round = Some(round_to_response(&round));
        }
    }

    let headline_available = pick_next_headline(cli).await?.is_some();

    let can_join = match &current_round {
        Some(r) => {
            !already_joined
                && (r.participant_pks.len() as i32) < settings.round_capacity
                && matches!(r.status, RoundStatus::Waiting)
        }
        None => headline_available,
    };

    Ok(LobbyResponse {
        current_round,
        can_join,
        already_joined,
        round_capacity: settings.round_capacity,
        min_bet_rp: settings.min_bet_rp,
        headline_available,
    })
}

// ── POST /api/fact-or-fold/lobby/join ─────────────────────────────

#[post("/api/fact-or-fold/lobby/join", user: User)]
pub async fn join_lobby_handler() -> Result<RoundResponse> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let settings = load_settings_or_default(cli).await;
    if user.points < settings.min_bet_rp {
        return Err(FactOrFoldError::LobbyInsufficientBalance(settings.min_bet_rp).into());
    }

    let mut lobby = FactFoldLobby::get_or_default(cli).await.map_err(|e| {
        crate::error!("join_lobby_handler lobby read failed: {e}");
        FactOrFoldError::StorageFailure
    })?;

    let user_pk_str = user.pk.to_string();

    // Try to attach to an existing waiting round.
    if let Some(round_id) = lobby.current_round_id.clone() {
        let (pk, sk) = FactFoldRound::keys(&round_id);
        let round = FactFoldRound::get(cli, &pk, Some(sk.clone()))
            .await
            .map_err(|e| {
                crate::error!("join_lobby_handler round read failed: {e}");
                FactOrFoldError::StorageFailure
            })?;
        if let Some(mut round) = round {
            if !matches!(round.status, RoundStatus::Waiting) {
                // Pointer is stale (e.g. round already started). Drop
                // the pointer so the next branch creates a new one.
                lobby.current_round_id = None;
            } else {
                if round
                    .participant_pks
                    .iter()
                    .any(|p| p.to_string() == user_pk_str)
                {
                    return Err(FactOrFoldError::LobbyAlreadyJoined.into());
                }
                if (round.participant_pks.len() as i32) >= settings.round_capacity {
                    return Err(FactOrFoldError::LobbyFull.into());
                }
                round.participant_pks.push(user.pk.clone());
                let now = crate::common::utils::time::get_now_timestamp_millis();
                let lobby_should_clear =
                    (round.participant_pks.len() as i32) >= settings.round_capacity;
                if lobby_should_clear {
                    round.status = RoundStatus::NewsReveal;
                    round.started_at = Some(now);
                }
                round.updated_at = now;
                round.upsert(cli).await.map_err(|e| {
                    crate::error!("join_lobby_handler round upsert failed: {e}");
                    FactOrFoldError::StorageFailure
                })?;
                if lobby_should_clear {
                    upsert_lobby_pointer(cli, None).await?;
                }
                return Ok(round_to_response(&round));
            }
        } else {
            // Pointer dangles — drop it.
            lobby.current_round_id = None;
        }
    }

    // No waiting round (or pointer was stale) — create a new one.
    let headline = pick_next_headline(cli)
        .await?
        .ok_or(FactOrFoldError::LobbyNoHeadlineAvailable)?;
    let headline_id = headline
        .id()
        .ok_or(FactOrFoldError::LobbyNoHeadlineAvailable)?;

    // Promote Scheduled → Live so the headline is bound to this
    // round and won't be picked again by a parallel matching call.
    if matches!(headline.status, HeadlineStatus::Scheduled) {
        let pk = FactFoldHeadline::anchor_pk();
        let sk: EntityType = FactFoldHeadlineEntityType(headline_id.clone()).into();
        let now = crate::common::utils::time::get_now_timestamp_millis();
        FactFoldHeadline::updater(&pk, &sk)
            .with_status(HeadlineStatus::Live)
            .with_updated_at(now)
            .execute(cli)
            .await
            .map_err(|e| {
                crate::error!("join_lobby_handler headline promote failed: {e}");
                FactOrFoldError::StorageFailure
            })?;
    }

    let round_id = uuid::Uuid::new_v4().to_string();
    let mut round = FactFoldRound::new_waiting(round_id.clone(), headline_id, user.pk.clone());

    // Single-user round is full when round_capacity == 1 — flip
    // straight to NewsReveal in that edge case.
    let lobby_should_clear = (round.participant_pks.len() as i32) >= settings.round_capacity;
    if lobby_should_clear {
        let now = crate::common::utils::time::get_now_timestamp_millis();
        round.status = RoundStatus::NewsReveal;
        round.started_at = Some(now);
        round.updated_at = now;
    }

    round.create(cli).await.map_err(|e| {
        crate::error!("join_lobby_handler round create failed: {e}");
        FactOrFoldError::StorageFailure
    })?;

    upsert_lobby_pointer(
        cli,
        if lobby_should_clear {
            None
        } else {
            Some(round_id.clone())
        },
    )
    .await?;

    Ok(round_to_response(&round))
}

// ── POST /api/fact-or-fold/lobby/leave ────────────────────────────

#[post("/api/fact-or-fold/lobby/leave", user: User)]
pub async fn leave_lobby_handler() -> Result<RoundResponse> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let lobby = FactFoldLobby::get_or_default(cli).await.map_err(|e| {
        crate::error!("leave_lobby_handler lobby read failed: {e}");
        FactOrFoldError::StorageFailure
    })?;

    let round_id = lobby
        .current_round_id
        .clone()
        .ok_or(FactOrFoldError::LobbyNotJoined)?;

    let (pk, sk) = FactFoldRound::keys(&round_id);
    let mut round = FactFoldRound::get(cli, &pk, Some(sk.clone()))
        .await
        .map_err(|e| {
            crate::error!("leave_lobby_handler round read failed: {e}");
            FactOrFoldError::StorageFailure
        })?
        .ok_or(FactOrFoldError::LobbyNotJoined)?;

    let user_pk_str = user.pk.to_string();
    let before_len = round.participant_pks.len();
    round
        .participant_pks
        .retain(|p| p.to_string() != user_pk_str);
    if round.participant_pks.len() == before_len {
        return Err(FactOrFoldError::LobbyNotJoined.into());
    }

    round.updated_at = crate::common::utils::time::get_now_timestamp_millis();
    round.upsert(cli).await.map_err(|e| {
        crate::error!("leave_lobby_handler round upsert failed: {e}");
        FactOrFoldError::StorageFailure
    })?;

    Ok(round_to_response(&round))
}

// ── GET /api/fact-or-fold/rounds/{round_id} ──────────────────────

#[get("/api/fact-or-fold/rounds/{round_id}", _user: User)]
pub async fn get_round_handler(round_id: FactFoldRoundEntityType) -> Result<RoundResponse> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let inner = round_id.0.clone();
    let (pk, sk) = FactFoldRound::keys(&inner);
    let round = FactFoldRound::get(cli, &pk, Some(sk))
        .await
        .map_err(|e| {
            crate::error!("get_round_handler read failed: {e}");
            FactOrFoldError::StorageFailure
        })?
        .ok_or(FactOrFoldError::RoundNotFound)?;
    Ok(round_to_response(&round))
}
