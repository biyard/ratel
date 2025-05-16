use std::{
    sync::{Arc, OnceLock},
    time::Duration,
};

use bdk::prelude::*;
use by_axum::{
    auth::Authorization,
    axum::{Extension, Json, extract::State, native_routing::post},
};
use dto::*;
use tokio::time::sleep;

use crate::modules::ch_parliament::ChParliamentClient;

#[derive(Clone, Debug)]
pub struct CHBillWriterController {
    repo: CHBillWriterRepository,
    cli: ChParliamentClient,
    pool: sqlx::Pool<sqlx::Postgres>,
}

static INSTANCE: OnceLock<bool> = OnceLock::new();
static CURRENT_YEAR: i64 = 2025;

impl CHBillWriterController {
    pub async fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = CHBillWriter::get_repository(pool.clone());
        let cli = ChParliamentClient::new();

        let ctrl = Self { repo, pool, cli };

        let arc = Arc::new(ctrl.clone());

        if INSTANCE.get().is_none() {
            let res = INSTANCE.set(true);
            if let Err(e) = res {
                tracing::error!("Failed to initialize INSTANCE on {e:?}");
            }
            tokio::spawn(async move {
                let _ = arc.fetch_recent_bills(CURRENT_YEAR).await;
                sleep(Duration::from_secs(60 * 60)).await;
            });
        }

        ctrl
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .native_route("/", post(Self::act_bill))
            .with_state(self.clone()))
    }

    pub async fn act_bill(
        State(ctrl): State<CHBillWriterController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Json(body): Json<CHBillWriterAction>,
    ) -> Result<Json<CHBillWriter>> {
        tracing::debug!("act_bill {:?}", body);
        let res = match body {
            CHBillWriterAction::FetchBills(param) => {
                ctrl.fetch_bills(param.year, param.start_bill_no, param.end_bill_no)
                    .await?
            }
            CHBillWriterAction::FetchRecentBills(param) => {
                ctrl.fetch_recent_bills(param.year).await?
            }
            CHBillWriterAction::FetchBill(param) => ctrl.fetch_bill(param).await?,
        };
        Ok(Json(res))
    }
}

impl CHBillWriterController {
    async fn fetch_bills(&self, year: i64, mut bill_no: i64, end: i64) -> Result<CHBillWriter> {
        tracing::debug!("fetch_bills {:?} {:?}", year, bill_no);
        let mut bill_ids: Vec<(i64, String)> = vec![];

        loop {
            for i in 0..3 {
                let bill_id = format!("{:04}{:04}", year, bill_no)
                    .parse::<i64>()
                    .unwrap_or_default();
                let res = self
                    .fetch_bill(CHBillWriterFetchBillRequest { bill_id })
                    .await;

                if res.is_ok() {
                    tracing::info!("fetched {} bill", bill_id);
                    break;
                } else {
                    tracing::error!("Failed to fetch bill {}: {:?}", bill_id, res);
                    if i == 2 {
                        bill_ids.push((bill_id, format!("{:?}", res)));
                    }
                }
            }
            bill_no += 1;
            if bill_no > end {
                break;
            }
        }

        if bill_ids.is_empty() {
            Ok(Default::default())
        } else {
            Err(Error::FetchError(bill_ids))
        }
    }

    async fn fetch_recent_bills(&self, year: i64) -> Result<CHBillWriter> {
        let CHBillWriter { mut bill_no, .. } = CHBillWriter::query_builder()
            .order_by_id_desc()
            .query()
            .map(CHBillWriter::from)
            .fetch_one(&self.pool)
            .await?;
        let mut bill = CHBillWriter::default();
        tracing::info!("last fetched bill: {:?}", bill_no);
        bill_no += 1;

        loop {
            let bill_id = format!("{:04}{:04}", year, bill_no)
                .parse::<i64>()
                .unwrap_or_default();
            let res = self
                .fetch_bill(CHBillWriterFetchBillRequest { bill_id })
                .await;
            tracing::debug!("fetched {:?}", res);

            if let Ok(b) = res {
                tracing::info!("fetched {}", bill_no);
                bill = b.clone();
                bill_no += 1;
            } else {
                tracing::debug!("no bill has been found");
                break;
            }
        }

        Ok(bill)
    }

    async fn fetch_bill(
        &self,
        CHBillWriterFetchBillRequest { bill_id }: CHBillWriterFetchBillRequest,
    ) -> Result<CHBillWriter> {
        tracing::debug!("fetch_bill {:?}", bill_id);

        let bill = self.cli.get_bill(bill_id).await?;

        let bill: CHBillWriter = bill.into();

        let bill = if let Some(b) = CHBillWriter::query_builder()
            .bill_id_equals(bill_id)
            .query()
            .map(CHBillWriter::from)
            .fetch_optional(&self.pool)
            .await?
        {
            let CHBillWriter {
                bill_id,
                year,
                bill_no,
                title,
                description,
                initial_situation,
                procedings,
                date,
                ..
            } = bill;
            self.repo
                .update(
                    b.id,
                    CHBillWriterRepositoryUpdateRequest {
                        bill_id: Some(bill_id),
                        year: Some(year),
                        bill_no: Some(bill_no),
                        title: Some(title),
                        description: Some(description),
                        initial_situation: Some(initial_situation),
                        procedings: Some(procedings),
                        date: Some(date),
                    },
                )
                .await?
        } else {
            self.repo
                .insert(
                    bill.bill_id,
                    bill.year,
                    bill.bill_no,
                    bill.title,
                    bill.description,
                    bill.initial_situation,
                    bill.procedings,
                    bill.date,
                )
                .await?
        };

        Ok(bill)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{TestContext, setup};

    #[tokio::test]
    async fn test_fetch_bill() {
        let TestContext { endpoint, .. } = setup().await.unwrap();

        let cli = CHBillWriter::get_client(&endpoint);

        let bill = cli.fetch_bill(20250001).await;

        assert!(bill.is_ok(), "Failed to fetch bill: {:?}", bill);
        let bill = bill.unwrap();

        assert_eq!(bill.bill_id, 20250001);
        assert_eq!(bill.title, "Rapport de gestion du Conseil fédéral 2024");
        assert_eq!(bill.year, 2025);
        assert_eq!(bill.bill_no, 1);
    }
}
