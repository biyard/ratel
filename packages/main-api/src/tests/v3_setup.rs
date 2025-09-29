use axum::AxumRouter;
use bdk::prelude::*;
use std::time::SystemTime;

use crate::api_main;

pub struct TestContextV3 {
    pub app: AxumRouter,
    pub now: u64,
}

pub async fn setup_v3() -> TestContextV3 {
    let app = api_main::api_main().await.unwrap();
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as u64
        - 1750000000u64;

    let app = by_axum::finishing(app);

    TestContextV3 { app, now }
}
