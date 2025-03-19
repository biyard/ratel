#[allow(unused_imports)]
use crate::utils::openapi::*;
use bdk::prelude::*;
use by_axum::axum::{extract::State, routing::post};
use dto::*;

const BATCH_SIZE: u32 = 10;
const MAX_BILL_SUM: u32 = 1500; // 2025.03.19: 1392
#[derive(Clone, Debug)]
pub struct BillsController {
    pub _pool: sqlx::Pool<sqlx::Postgres>,
    pub repo: BillRepository,
}

impl BillsController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = Bill::get_repository(pool.clone());
        Self { _pool: pool, repo }
    }

    pub fn route(&self) -> by_axum::axum::Router {
        by_axum::axum::Router::new()
            .route("/", post(Self::act_bills))
            .with_state(self.clone())
    }

    pub async fn act_bills(State(ctrl): State<BillsController>) -> Result<()> {
        ctrl.fetch_bills().await?;

        Ok(())
    }
}

impl BillsController {
    pub async fn fetch_bills(&self) -> Result<()> {
        for i in 1..=MAX_BILL_SUM / BATCH_SIZE {
            let bills = fetch_bills(i, BATCH_SIZE).await?;
            tracing::debug!("bills: {:?}", bills);

            for bill in bills {
                let file_book_id = get_file_book_id(bill.site_link.clone()).await?;
                tracing::debug!("file_book_id: {:?}", file_book_id);

                match self
                    .repo
                    .insert(
                        bill.bill_no,
                        bill.bill_id,
                        bill.title,
                        file_book_id,
                        bill.site_link,
                        None,
                        None,
                        None,
                    )
                    .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        tracing::error!("error: {:?}", e);
                        break;
                    }
                }
            }
        }
        Ok(())
    }
}
