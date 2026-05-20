//! `UseFactFoldRound` — game-room controller.
//!
//! Bundles every read + mutation the round-stage UI needs into one
//! context-cached hook. The page calls `use_fact_fold_round_provider`
//! with the `round_id` from the route; every sub-component (news
//! reveal / first bet / reasoning write / reveal / debate /
//! settlement) reads from this controller. Components never call
//! the `_handler` server functions directly.
//!
//! Polling: the page-level component runs a `use_future` tick that
//! bumps every `*_refresh` signal every ~2.5s so each loader re-fires
//! its fetch; `poll_chat()` handles incremental chat deltas.
//! Mutations bump only the loader(s) affected by the mutation so the
//! UI flips without waiting for the next tick.
//!
//! Loader-resolution convention (dev memo 2026-05-19): loaders are
//! exposed as methods returning `Result<Loader<T>, Loading>`. The
//! provider stores only refresh signals so it doesn't suspend the
//! whole subtree; each consuming component `?`-suspends at its own
//! call site, letting siblings render independently.

use crate::features::arcade::games::fact_or_fold::controllers::essence::{
    register_essence_handler, RegisterEssenceRequest,
};
use crate::features::arcade::games::fact_or_fold::controllers::settlement::SettleRoundResponse;
use crate::features::arcade::games::fact_or_fold::{
    delete_round_chat_handler, flip_bet_handler, get_insider_statement_handler, get_round_handler,
    get_round_subject_handler, get_round_settlement_handler, heartbeat_handler,
    list_chat_handler, list_round_bets_handler, list_round_participants_handler,
    list_round_rationales_handler, place_bet_handler, post_chat_handler,
    submit_rationale_handler, tick_handler, BetResponse, BetSide, ChatMessagePayload,
    FlipBetRequest, InsiderStatementResponse, ListBetsResponse, ListParticipantsResponse,
    ListRationalesResponse, PlaceBetRequest, PostChatRequest, RoundSubjectResponse,
    RoundResponse, SubmitRationaleRequest,
};
use crate::FactFoldRoundEntityType;
use crate::*;

#[derive(Clone, Copy, DioxusController)]
pub struct UseFactFoldRound {
    /// Active round id — referenced by every loader method.
    pub round_id: ReadSignal<FactFoldRoundEntityType>,

    // Per-loader refresh triggers. Each loader method reads its
    // refresh signal inside the future closure; `use_resource`'s
    // reactive tracking re-fires the fetch when the signal bumps.
    pub round_refresh: Signal<u64>,
    pub subject_refresh: Signal<u64>,
    pub participants_refresh: Signal<u64>,
    pub bets_refresh: Signal<u64>,
    pub rationales_refresh: Signal<u64>,
    pub insider_refresh: Signal<u64>,
    pub settlement_refresh: Signal<u64>,

    /// Chronologically ordered chat log, appended only — `poll_chat`
    /// asks the server for messages newer than `last_chat_id`.
    pub chat: Signal<Vec<ChatMessagePayload>>,
    pub last_chat_id: Signal<Option<String>>,
    /// User id that the caller marked as "decisive" during the reveal
    /// stage (mockup's ⌬ icon). Carries the citation across into the
    /// live-debate flip slot — `flip_bet` uses this as the default
    /// cite_user_pk. Cleared when the round terminates.
    pub cited_user_pk: Signal<Option<UserPartition>>,
}

impl UseFactFoldRound {
    // ── Loader accessors ─────────────────────────────────────────

    pub fn round(&self) -> std::result::Result<Loader<RoundResponse>, Loading> {
        let round_id = self.round_id;
        let refresh = self.round_refresh;
        use_loader(move || {
            let _ = refresh();
            let rid = round_id();
            async move { get_round_handler(rid).await }
        })
    }

    pub fn subject(&self) -> std::result::Result<Loader<RoundSubjectResponse>, Loading> {
        let round_id = self.round_id;
        let refresh = self.subject_refresh;
        use_loader(move || {
            let _ = refresh();
            let rid = round_id();
            async move { get_round_subject_handler(rid).await }
        })
    }

    pub fn participants(&self) -> std::result::Result<Loader<ListParticipantsResponse>, Loading> {
        let round_id = self.round_id;
        let refresh = self.participants_refresh;
        use_loader(move || {
            let _ = refresh();
            let rid = round_id();
            async move { list_round_participants_handler(rid).await }
        })
    }

    pub fn bets(&self) -> std::result::Result<Loader<ListBetsResponse>, Loading> {
        let round_id = self.round_id;
        let refresh = self.bets_refresh;
        use_loader(move || {
            let _ = refresh();
            let rid = round_id();
            async move { list_round_bets_handler(rid).await }
        })
    }

    pub fn rationales(&self) -> std::result::Result<Loader<ListRationalesResponse>, Loading> {
        let round_id = self.round_id;
        let refresh = self.rationales_refresh;
        use_loader(move || {
            let _ = refresh();
            let rid = round_id();
            async move { list_round_rationales_handler(rid).await }
        })
    }

    pub fn insider(&self) -> std::result::Result<Loader<InsiderStatementResponse>, Loading> {
        let round_id = self.round_id;
        let refresh = self.insider_refresh;
        use_loader(move || {
            let _ = refresh();
            let rid = round_id();
            async move { get_insider_statement_handler(rid).await }
        })
    }

    pub fn settlement(
        &self,
    ) -> std::result::Result<Loader<Option<SettleRoundResponse>>, Loading> {
        let round_id = self.round_id;
        let refresh = self.settlement_refresh;
        use_loader(move || {
            let _ = refresh();
            let rid = round_id();
            // Settlement GET returns 409 RoundNotSettled before the
            // round wraps up. Treat that as "not ready yet" so the
            // loader keeps returning Ok(None) instead of flipping
            // into an error state.
            async move {
                match get_round_settlement_handler(rid).await {
                    Ok(r) => Ok::<_, crate::common::Error>(Some(r)),
                    Err(_) => Ok(None),
                }
            }
        })
    }

    // ── Refresh helpers ──────────────────────────────────────────

    /// Refresh every round-scoped loader. Called by the page's polling
    /// future and after any mutation that changes server state. Chat
    /// is intentionally excluded — it is incremental, not replace-all.
    pub fn refresh_all(&mut self) {
        self.round_refresh.with_mut(|n| *n += 1);
        self.subject_refresh.with_mut(|n| *n += 1);
        self.participants_refresh.with_mut(|n| *n += 1);
        self.bets_refresh.with_mut(|n| *n += 1);
        self.rationales_refresh.with_mut(|n| *n += 1);
        self.insider_refresh.with_mut(|n| *n += 1);
        self.settlement_refresh.with_mut(|n| *n += 1);
    }

    // ── Stage advance ─────────────────────────────────────────────

    pub async fn tick(&mut self, round_id: FactFoldRoundEntityType) -> crate::common::Result<()> {
        let _ = tick_handler(round_id).await?;
        // Stage may have advanced — refresh everything dependent on it.
        self.refresh_all();
        Ok(())
    }

    pub async fn heartbeat(
        &mut self,
        round_id: FactFoldRoundEntityType,
    ) -> crate::common::Result<()> {
        let _ = heartbeat_handler(round_id).await?;
        Ok(())
    }

    // ── Player mutations ──────────────────────────────────────────

    pub async fn place_bet(
        &mut self,
        round_id: FactFoldRoundEntityType,
        side: BetSide,
        amount_rp: i64,
    ) -> crate::common::Result<BetResponse> {
        let res = place_bet_handler(round_id, PlaceBetRequest { side, amount_rp }).await?;
        self.bets_refresh.with_mut(|n| *n += 1);
        Ok(res)
    }

    pub async fn submit_rationale(
        &mut self,
        round_id: FactFoldRoundEntityType,
        text: String,
    ) -> crate::common::Result<()> {
        let _ = submit_rationale_handler(round_id, SubmitRationaleRequest { text }).await?;
        self.rationales_refresh.with_mut(|n| *n += 1);
        Ok(())
    }

    pub async fn register_essence(
        &mut self,
        round_id: FactFoldRoundEntityType,
    ) -> crate::common::Result<()> {
        let _ = register_essence_handler(round_id, RegisterEssenceRequest { register: true })
            .await?;
        self.rationales_refresh.with_mut(|n| *n += 1);
        Ok(())
    }

    pub async fn flip_bet(
        &mut self,
        round_id: FactFoldRoundEntityType,
        side: BetSide,
        cite_user_pk: UserPartition,
    ) -> crate::common::Result<()> {
        let _ = flip_bet_handler(round_id, FlipBetRequest { side, cite_user_pk }).await?;
        self.bets_refresh.with_mut(|n| *n += 1);
        Ok(())
    }

    // ── Chat ──────────────────────────────────────────────────────

    pub async fn post_chat(
        &mut self,
        round_id: FactFoldRoundEntityType,
        text: String,
    ) -> crate::common::Result<()> {
        let res = post_chat_handler(round_id, PostChatRequest { text }).await?;
        // Optimistically append so the sender sees their own message
        // before the next poll cycle. The polling fetch will dedup via
        // `last_chat_id` so this row won't be added twice.
        let msg = ChatMessagePayload {
            msg_id: res.msg_id.clone(),
            author_pk: res.author_pk,
            text: res.text,
            sent_at: res.sent_at,
        };
        let mut buf = self.chat.write();
        if !buf.iter().any(|m| m.msg_id == msg.msg_id) {
            buf.push(msg);
        }
        // Don't touch last_chat_id — the next poll will still see this
        // row server-side and increment `last_chat_id` itself, keeping
        // the polling cursor consistent across sessions.
        Ok(())
    }

    /// Bulk-delete the round's chat transcript. Called from the
    /// settlement screen's "정산완료 → 홈으로" button. Local chat
    /// buffer is also cleared so the SettlementView can navigate
    /// away without flashing leftover messages.
    pub async fn exit_round(
        &mut self,
        round_id: FactFoldRoundEntityType,
    ) -> crate::common::Result<()> {
        let _ = delete_round_chat_handler(round_id).await?;
        self.chat.set(Vec::new());
        self.last_chat_id.set(None);
        Ok(())
    }

    /// Pull any chat messages newer than `last_chat_id` and append
    /// them to the visible buffer. Called by the page's polling tick.
    pub async fn poll_chat(
        &mut self,
        round_id: FactFoldRoundEntityType,
    ) -> crate::common::Result<()> {
        let since = self.last_chat_id.read().clone();
        let res = list_chat_handler(round_id, since).await?;
        if res.items.is_empty() {
            return Ok(());
        }
        let mut buf = self.chat.write();
        for m in res.items.iter() {
            if !buf.iter().any(|existing| existing.msg_id == m.msg_id) {
                buf.push(m.clone());
            }
        }
        if let Some(last) = res.last_id {
            self.last_chat_id.set(Some(last));
        }
        Ok(())
    }
}

#[track_caller]
pub fn use_fact_fold_round_provider(
    round_id: ReadSignal<FactFoldRoundEntityType>,
) -> std::result::Result<UseFactFoldRound, RenderError> {
    if let Some(ctx) = try_use_context::<UseFactFoldRound>() {
        return Ok(ctx);
    }

    let round_refresh = use_signal(|| 0u64);
    let subject_refresh = use_signal(|| 0u64);
    let participants_refresh = use_signal(|| 0u64);
    let bets_refresh = use_signal(|| 0u64);
    let rationales_refresh = use_signal(|| 0u64);
    let insider_refresh = use_signal(|| 0u64);
    let settlement_refresh = use_signal(|| 0u64);

    let chat = use_signal(Vec::<ChatMessagePayload>::new);
    let last_chat_id = use_signal(|| None::<String>);
    let cited_user_pk = use_signal(|| None::<UserPartition>);

    Ok(use_context_provider(|| UseFactFoldRound {
        round_id,
        round_refresh,
        subject_refresh,
        participants_refresh,
        bets_refresh,
        rationales_refresh,
        insider_refresh,
        settlement_refresh,
        chat,
        last_chat_id,
        cited_user_pk,
    }))
}

pub fn use_fact_fold_round() -> UseFactFoldRound {
    use_context::<UseFactFoldRound>()
}
