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
//! calls `refresh_all()` every ~2.5s for stage / participants / bets
//! / rationale / settlement and `poll_chat()` for incremental chat
//! deltas. Mutations refresh the loaders that observe the row that
//! just changed so the UI flips without waiting for the next tick.

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
    pub round: Loader<RoundResponse>,
    pub subject: Loader<RoundSubjectResponse>,
    pub participants: Loader<ListParticipantsResponse>,
    pub bets: Loader<ListBetsResponse>,
    pub rationales: Loader<ListRationalesResponse>,
    pub insider: Loader<InsiderStatementResponse>,
    /// `None` while the round has not been settled yet (the GET 409s
    /// before the settlement row exists). Flips to `Some(breakdown)`
    /// once tick auto-settle or admin settle has run.
    pub settlement: Loader<Option<SettleRoundResponse>>,
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
    /// Refresh every round-scoped loader. Called by the page's polling
    /// future and after any mutation that changes server state. Chat
    /// is intentionally excluded — it is incremental, not replace-all.
    pub fn refresh_all(&mut self) {
        self.round.restart();
        self.subject.restart();
        self.participants.restart();
        self.bets.restart();
        self.rationales.restart();
        self.insider.restart();
        self.settlement.restart();
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
        self.bets.restart();
        Ok(res)
    }

    pub async fn submit_rationale(
        &mut self,
        round_id: FactFoldRoundEntityType,
        text: String,
    ) -> crate::common::Result<()> {
        let _ = submit_rationale_handler(round_id, SubmitRationaleRequest { text }).await?;
        self.rationales.restart();
        Ok(())
    }

    pub async fn register_essence(
        &mut self,
        round_id: FactFoldRoundEntityType,
    ) -> crate::common::Result<()> {
        let _ = register_essence_handler(round_id, RegisterEssenceRequest { register: true })
            .await?;
        self.rationales.restart();
        Ok(())
    }

    pub async fn flip_bet(
        &mut self,
        round_id: FactFoldRoundEntityType,
        side: BetSide,
        cite_user_pk: UserPartition,
    ) -> crate::common::Result<()> {
        let _ = flip_bet_handler(round_id, FlipBetRequest { side, cite_user_pk }).await?;
        self.bets.restart();
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

    let round = use_loader(move || async move { get_round_handler(round_id()).await })?;
    let subject =
        use_loader(move || async move { get_round_subject_handler(round_id()).await })?;
    let participants =
        use_loader(move || async move { list_round_participants_handler(round_id()).await })?;
    let bets = use_loader(move || async move { list_round_bets_handler(round_id()).await })?;
    let rationales =
        use_loader(move || async move { list_round_rationales_handler(round_id()).await })?;
    let insider =
        use_loader(move || async move { get_insider_statement_handler(round_id()).await })?;
    let settlement = use_loader(move || async move {
        // Settlement GET returns 409 RoundNotSettled before the round
        // wraps up. Treat that as "not ready yet" so the loader keeps
        // returning Ok(None) instead of flipping into an error state.
        match get_round_settlement_handler(round_id()).await {
            Ok(r) => Ok::<_, crate::common::Error>(Some(r)),
            Err(_) => Ok(None),
        }
    })?;

    let chat = use_signal(Vec::<ChatMessagePayload>::new);
    let last_chat_id = use_signal(|| None::<String>);
    let cited_user_pk = use_signal(|| None::<UserPartition>);

    Ok(use_context_provider(|| UseFactFoldRound {
        round,
        subject,
        participants,
        bets,
        rationales,
        insider,
        settlement,
        chat,
        last_chat_id,
        cited_user_pk,
    }))
}

pub fn use_fact_fold_round() -> UseFactFoldRound {
    use_context::<UseFactFoldRound>()
}
