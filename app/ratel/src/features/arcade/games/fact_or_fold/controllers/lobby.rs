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
//!      subject; transition it to Live).
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
use crate::features::arcade::games::fact_or_fold::types::*;

#[cfg(feature = "server")]
use crate::common::models::auth::User;
#[cfg(feature = "server")]
use crate::features::arcade::games::fact_or_fold::models::{
    FactFoldSubject, FactFoldLobby, FactFoldParticipant, FactFoldRound, FactFoldSettings,
};
#[cfg(feature = "server")]
use crate::features::arcade::games::fact_or_fold::services::stage_machine;
#[cfg(feature = "server")]
use crate::features::arcade::models::ArcadeSettings;
#[cfg(feature = "server")]
use crate::features::arcade::wallet::{ArcadeWallet, DdbArcadeWallet};

#[cfg(feature = "server")]
fn user_inner_id(user: &User) -> String {
    UserPartition::from(user.pk.clone()).0
}

// ── Internal helpers ──────────────────────────────────────────────

#[cfg(feature = "server")]
async fn load_settings_or_default(cli: &aws_sdk_dynamodb::Client) -> FactOrFoldSettingsResponse {
    FactFoldSettings::get_or_default(cli)
        .await
        .unwrap_or_default()
}

/// Pick the next eligible subject for a new round.
///
/// Uses the GSI3 (status, pick_at) index so the DB hands back already-sorted
/// FIFO order — no in-memory scan or sort. Two limit-1 queries:
///
/// 1. `Live` partition — any already-activated subject not yet bound to a
///    round wins immediately (oldest by `pick_at`).
/// 2. `Scheduled` partition — the oldest scheduled row is usable only when
///    its `pick_at` (= `scheduled_at`) is past-due. Because the GSI hands
///    back rows ASC, the first row's `pick_at` is the minimum: if it's still
///    in the future, every other Scheduled row is too.
#[cfg(feature = "server")]
async fn pick_next_subject(
    cli: &aws_sdk_dynamodb::Client,
) -> crate::common::Result<Option<FactFoldSubject>> {
    let live_opts = FactFoldSubject::opt().limit(1).oldest();
    let (live, _) =
        FactFoldSubject::find_by_status(cli, SubjectStatus::Live, live_opts)
            .await
            .map_err(|e| {
                crate::error!("pick_next_subject live query failed: {e}");
                FactOrFoldError::StorageFailure
            })?;
    if let Some(row) = live.into_iter().next() {
        return Ok(Some(row));
    }

    let now = crate::common::utils::time::get_now_timestamp_millis();
    let sched_opts = FactFoldSubject::opt().limit(1).oldest();
    let (sched, _) =
        FactFoldSubject::find_by_status(cli, SubjectStatus::Scheduled, sched_opts)
            .await
            .map_err(|e| {
                crate::error!("pick_next_subject scheduled query failed: {e}");
                FactOrFoldError::StorageFailure
            })?;
    Ok(sched.into_iter().find(|row| row.pick_at <= now))
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

/// Pick the insider index uniformly at random over `n` participants.
/// Pulled out so future deterministic-test overrides have a single
/// hook to swap.
#[cfg(feature = "server")]
fn pick_insider_index(n: usize) -> usize {
    use rand::RngExt;
    rand::rng().random_range(0..n)
}

/// Materialize FactFoldParticipant rows for every player in a
/// freshly-started round. Exactly one row is marked is_insider.
#[cfg(feature = "server")]
async fn create_participants_for_round(
    cli: &aws_sdk_dynamodb::Client,
    round_id: &str,
    participant_pks: &[Partition],
) -> crate::common::Result<()> {
    if participant_pks.is_empty() {
        return Ok(());
    }
    let insider_idx = pick_insider_index(participant_pks.len());
    for (idx, user_pk) in participant_pks.iter().enumerate() {
        let row = FactFoldParticipant::new(round_id, user_pk.clone(), idx == insider_idx);
        row.create(cli).await.map_err(|e| {
            crate::error!("create_participants_for_round failed for {user_pk:?}: {e}");
            FactOrFoldError::StorageFailure
        })?;
    }
    Ok(())
}

// ── GET /api/fact-or-fold/lobby ───────────────────────────────────

#[get("/api/fact-or-fold/lobby", user: User)]
pub async fn get_lobby_handler() -> Result<LobbyResponse> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    // Independent reads — kick them off in parallel so the four DDB
    // round-trips collapse to one. `current_round` depends on
    // `lobby.current_round_id` so it's loaded after this join.
    let settings_fut = async { Ok::<_, crate::common::Error>(load_settings_or_default(cli).await) };
    let lobby_fut = async {
        FactFoldLobby::get_or_default(cli).await.map_err(|e| {
            crate::error!("get_lobby_handler lobby read failed: {e}");
            crate::common::Error::from(FactOrFoldError::StorageFailure)
        })
    };
    let arcade_fut = async {
        Ok::<_, crate::common::Error>(
            ArcadeSettings::get_or_default(cli)
                .await
                .unwrap_or_default(),
        )
    };
    let pick_fut = async { pick_next_subject(cli).await.map(|s| s.is_some()) };
    let (settings, lobby, arcade_settings, subject_available) =
        tokio::try_join!(settings_fut, lobby_fut, arcade_fut, pick_fut)?;

    let mut current_round: Option<RoundResponse> = None;
    let mut already_joined = false;

    if let Some(round_id) = lobby.current_round_id.as_deref() {
        let (pk, sk) = FactFoldRound::keys(round_id);
        if let Some(round) = FactFoldRound::get(cli, &pk, Some(sk)).await.map_err(|e| {
            crate::error!("get_lobby_handler round read failed: {e}");
            FactOrFoldError::StorageFailure
        })? {
            already_joined = round.participant_pks.iter().any(|p| p == &user.pk);
            current_round = Some(RoundResponse::from(&round));
        }
    }

    let can_join = match &current_round {
        Some(r) => {
            !already_joined
                && (r.participant_pks.len() as i32) < settings.round_capacity
                && matches!(r.status, RoundStatus::Waiting)
        }
        None => subject_available,
    };

    Ok(LobbyResponse {
        current_round,
        can_join,
        already_joined,
        round_capacity: settings.round_capacity,
        min_bet_rp: settings.min_bet_rp,
        buy_in_chips: arcade_settings.default_buy_in_chips,
        subject_available,
    })
}

// ── POST /api/fact-or-fold/lobby/join ─────────────────────────────

#[post("/api/fact-or-fold/lobby/join", user: User)]
pub async fn join_lobby_handler() -> Result<RoundResponse> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let wallet = DdbArcadeWallet::new(cli.clone());
    let user_id = user_inner_id(&user);

    // Settings, lobby, wallet balance and arcade settings are all
    // independent reads — fan them out in parallel before applying
    // the chip-balance gate.
    let settings_fut = async { Ok::<_, crate::common::Error>(load_settings_or_default(cli).await) };
    let arcade_fut = async {
        Ok::<_, crate::common::Error>(
            ArcadeSettings::get_or_default(cli)
                .await
                .unwrap_or_default(),
        )
    };
    let balance_fut = async { wallet.balance(&user_id).await };
    let lobby_fut = async {
        FactFoldLobby::get_or_default(cli).await.map_err(|e| {
            crate::error!("join_lobby_handler lobby read failed: {e}");
            crate::common::Error::from(FactOrFoldError::StorageFailure)
        })
    };
    let (settings, arcade_settings, chip_balance, mut lobby) =
        tokio::try_join!(settings_fut, arcade_fut, balance_fut, lobby_fut)?;

    let buy_in_chips = arcade_settings.default_buy_in_chips;
    if chip_balance < buy_in_chips {
        return Err(crate::features::arcade::ArcadeError::WalletInsufficientChip.into());
    }

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
                if round.participant_pks.iter().any(|p| p == &user.pk) {
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
                    stage_machine::stamp_initial_stage(&mut round, &settings, now);
                }
                round.updated_at = now;
                round.upsert(cli).await.map_err(|e| {
                    crate::error!("join_lobby_handler round upsert failed: {e}");
                    FactOrFoldError::StorageFailure
                })?;
                let attached_round_id = round.id().unwrap_or_default();
                if lobby_should_clear {
                    create_participants_for_round(cli, &attached_round_id, &round.participant_pks)
                        .await?;
                    upsert_lobby_pointer(cli, None).await?;
                }
                // Lock chips on the table for this round. Anything
                // that happens inside the round is off-wallet until
                // settlement (design doc § A5).
                wallet
                    .buy_in(&user_id, &attached_round_id, buy_in_chips)
                    .await?;
                return Ok(RoundResponse::from(&round));
            }
        } else {
            // Pointer dangles — drop it.
            lobby.current_round_id = None;
        }
    }

    // No waiting round (or pointer was stale) — create a new one.
    let subject = pick_next_subject(cli)
        .await?
        .ok_or(FactOrFoldError::LobbyNoSubjectAvailable)?;
    let subject_id = subject
        .id()
        .ok_or(FactOrFoldError::LobbyNoSubjectAvailable)?;

    // Promote Scheduled → Live so the subject is bound to this
    // round and won't be picked again by a parallel matching call.
    if matches!(subject.status, SubjectStatus::Scheduled) {
        let pk = FactFoldSubject::anchor_pk();
        let sk: EntityType = FactFoldSubjectEntityType(subject_id.clone()).into();
        let now = crate::common::utils::time::get_now_timestamp_millis();
        FactFoldSubject::updater(&pk, &sk)
            .with_status(SubjectStatus::Live)
            .with_updated_at(now)
            .execute(cli)
            .await
            .map_err(|e| {
                crate::error!("join_lobby_handler subject promote failed: {e}");
                FactOrFoldError::StorageFailure
            })?;
    }

    let round_id = uuid::Uuid::now_v7().to_string();
    let mut round = FactFoldRound::new_waiting(round_id.clone(), subject_id, user.pk.clone());

    // Single-user round is full when round_capacity == 1 — flip
    // straight to NewsReveal in that edge case.
    let lobby_should_clear = (round.participant_pks.len() as i32) >= settings.round_capacity;
    if lobby_should_clear {
        let now = crate::common::utils::time::get_now_timestamp_millis();
        round.status = RoundStatus::NewsReveal;
        round.started_at = Some(now);
        stage_machine::stamp_initial_stage(&mut round, &settings, now);
        round.updated_at = now;
    }

    round.create(cli).await.map_err(|e| {
        crate::error!("join_lobby_handler round create failed: {e}");
        FactOrFoldError::StorageFailure
    })?;

    if lobby_should_clear {
        create_participants_for_round(cli, &round_id, &round.participant_pks).await?;
    }

    upsert_lobby_pointer(
        cli,
        if lobby_should_clear {
            None
        } else {
            Some(round_id.clone())
        },
    )
    .await?;

    // Lock chips on the table for this round.
    wallet.buy_in(&user_id, &round_id, buy_in_chips).await?;

    Ok(RoundResponse::from(&round))
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

    let before_len = round.participant_pks.len();
    round.participant_pks.retain(|p| p != &user.pk);
    if round.participant_pks.len() == before_len {
        return Err(FactOrFoldError::LobbyNotJoined.into());
    }

    round.updated_at = crate::common::utils::time::get_now_timestamp_millis();
    round.upsert(cli).await.map_err(|e| {
        crate::error!("leave_lobby_handler round upsert failed: {e}");
        FactOrFoldError::StorageFailure
    })?;

    // Refund the buy-in. Leave is only valid while the round is
    // still Waiting, so the player gets every chip back.
    let arcade_settings = ArcadeSettings::get_or_default(cli)
        .await
        .unwrap_or_default();
    let wallet = DdbArcadeWallet::new(cli.clone());
    let user_id = user_inner_id(&user);
    wallet
        .settle(&user_id, &round_id, arcade_settings.default_buy_in_chips)
        .await?;

    Ok(RoundResponse::from(&round))
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

    // Lazy advance: any read of a round ratchets it through any
    // stages whose deadline has already passed. PR4 follow-ups add
    // a scheduled EventBridge trigger as the primary path; this
    // stays as a safety net so a stale client read still observes
    // the correct stage (§FR-9).
    let settings = load_settings_or_default(cli).await;
    let now = crate::common::utils::time::get_now_timestamp_millis();
    let round = stage_machine::advance_round_if_due(cli, round, &settings, now).await?;
    Ok(RoundResponse::from(&round))
}
