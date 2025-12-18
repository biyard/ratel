use crate::*;
use bdk::prelude::JsonSchema;
use by_axum::aide::OperationIo;
use by_axum::aide::axum::routing::{get, post};
use by_axum::axum::{
    Extension, Json, Router,
    extract::{Query, State},
};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::{
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::{Duration, Instant},
};
use uuid::Uuid;

use crate::AppState;

pub const TARGET_PAGE_KEY: &str = "/spaces/SPACE%23019abfa5-c420-72b2-8220-45f3918b001e/polls/SPACE_POLL%23019ac017-a6bd-74e2-9d01-2f75b0fc300d";

fn normalize_page_key(input: &str) -> String {
    let mut s = input.trim().to_string();

    if s.starts_with("http://") || s.starts_with("https://") {
        if let Some(idx) = s.find("://") {
            if let Some(slash) = s[idx + 3..].find('/') {
                s = s[idx + 3 + slash..].to_string();
            }
        }
    }

    if !s.starts_with('/') {
        s.insert(0, '/');
    }

    if let Some(q) = s.find('?') {
        s.truncate(q);
    }
    if let Some(h) = s.find('#') {
        s.truncate(h);
    }

    s
}

#[derive(Debug, Clone)]
pub struct PresenceState {
    sessions: Arc<DashMap<String, Instant>>,
    window: Duration,
    peak: Arc<AtomicUsize>,
}

impl PresenceState {
    pub fn new(window: Duration) -> Self {
        Self {
            sessions: Arc::new(DashMap::new()),
            window,
            peak: Arc::new(AtomicUsize::new(0)),
        }
    }

    fn cleanup(&self) {
        let now = Instant::now();
        self.sessions
            .retain(|_, last| now.duration_since(*last) <= self.window);
    }

    fn current(&self) -> usize {
        self.cleanup();
        self.sessions.len()
    }

    fn update_peak(&self, current: usize) -> usize {
        loop {
            let p = self.peak.load(Ordering::SeqCst);
            if current <= p {
                return p;
            }
            if self
                .peak
                .compare_exchange(p, current, Ordering::SeqCst, Ordering::SeqCst)
                .is_ok()
            {
                return current;
            }
        }
    }

    fn reset(&self) {
        self.sessions.clear();
        self.peak.store(0, Ordering::SeqCst);
    }
}

#[derive(Debug, Clone, Deserialize, OperationIo, JsonSchema)]
pub struct StartReq {
    pub page_key: String,
}

#[derive(Debug, Clone, Serialize, OperationIo, JsonSchema)]
pub struct StartRes {
    pub session_id: String,
    pub window_seconds: u64,
}

pub async fn start(
    State(_app): State<AppState>,
    Extension(st): Extension<PresenceState>,
    Json(req): Json<StartReq>,
) -> Result<Json<StartRes>> {
    let key = normalize_page_key(&req.page_key);

    if key != TARGET_PAGE_KEY {
        return Ok(Json(StartRes {
            session_id: "IGNORED".into(),
            window_seconds: st.window.as_secs(),
        }));
    }

    let sid = Uuid::new_v4().to_string();
    st.sessions.insert(sid.clone(), Instant::now());

    let cur = st.current();
    st.update_peak(cur);

    Ok(Json(StartRes {
        session_id: sid,
        window_seconds: st.window.as_secs(),
    }))
}

#[derive(Debug, Clone, Deserialize, OperationIo, JsonSchema)]
pub struct PingReq {
    pub session_id: String,
    pub page_key: String,
}

#[derive(Debug, Clone, Serialize, OperationIo, JsonSchema)]
pub struct PingRes {
    pub ok: bool,
}

pub async fn ping(
    State(_app): State<AppState>,
    Extension(st): Extension<PresenceState>,
    Json(req): Json<PingReq>,
) -> Result<Json<PingRes>> {
    let key = normalize_page_key(&req.page_key);

    if key != TARGET_PAGE_KEY || req.session_id == "IGNORED" {
        return Ok(Json(PingRes { ok: false }));
    }

    if let Some(mut e) = st.sessions.get_mut(&req.session_id) {
        *e = Instant::now();

        let cur = st.current();
        st.update_peak(cur);

        Ok(Json(PingRes { ok: true }))
    } else {
        Ok(Json(PingRes { ok: false }))
    }
}

#[derive(Debug, Clone, Deserialize, OperationIo, JsonSchema)]
pub struct StatsQuery {
    pub reset: Option<bool>,
}

#[derive(Debug, Clone, Serialize, OperationIo, JsonSchema)]
pub struct StatsRes {
    pub page_key: &'static str,
    pub current: usize,
    pub peak: usize,
    pub window_seconds: u64,
}

pub async fn stats(
    State(_app): State<AppState>,
    Extension(st): Extension<PresenceState>,
    Query(q): Query<StatsQuery>,
) -> Result<Json<StatsRes>> {
    if q.reset == Some(true) {
        st.reset();
    }

    let current = st.current();
    let peak = st.update_peak(current);

    Ok(Json(StatsRes {
        page_key: TARGET_PAGE_KEY,
        current,
        peak,
        window_seconds: st.window.as_secs(),
    }))
}

pub fn route() -> Result<Router<AppState>> {
    let st = PresenceState::new(Duration::from_secs(30));

    Ok(Router::<AppState>::new()
        .route("/start", post(start))
        .route("/ping", post(ping))
        .route("/stats", get(stats))
        .layer(Extension(st)))
}
