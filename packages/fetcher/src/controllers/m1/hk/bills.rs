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

use crate::modules::hk_council::HkCouncilClient;

#[derive(Clone, Debug)]
pub struct HKBillWriterController {
    repo: HKBillWriterRepository,
    cli: HkCouncilClient,
    pool: sqlx::Pool<sqlx::Postgres>,
}

static INSTANCE: OnceLock<bool> = OnceLock::new();
static MAX_BILL_ID: i64 = 9999;

impl HKBillWriterController {
    pub async fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = HKBillWriter::get_repository(pool.clone());
        let cli = HkCouncilClient::new();

        let ctrl = Self { repo, pool, cli };

        let arc = Arc::new(ctrl.clone());

        if INSTANCE.get().is_none() {
            let res = INSTANCE.set(true);
            if let Err(e) = res {
                tracing::error!("Failed to initialize INSTANCE on {e:?}");
            }
            tokio::spawn(async move {
                let _ = arc.fetch_recent_bills().await;
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
        State(ctrl): State<HKBillWriterController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Json(body): Json<HKBillWriterAction>,
    ) -> Result<Json<HKBillWriter>> {
        tracing::debug!("act_bill {:?}", body);
        let res = match body {
            HKBillWriterAction::FetchBills(param) => {
                ctrl.fetch_bills(param.start_bill_no, param.end_bill_no)
                    .await?
            }
            HKBillWriterAction::FetchRecentBills(_) => ctrl.fetch_recent_bills().await?,
            HKBillWriterAction::FetchBill(param) => ctrl.fetch_bill(param).await?,
        };
        Ok(Json(res))
    }
}

impl HKBillWriterController {
    async fn fetch_bills(&self, mut bill_id: i64, end: i64) -> Result<HKBillWriter> {
        tracing::debug!("fetch_bills {:?}", bill_id);
        let mut bill_ids: Vec<(i64, String)> = vec![];

        loop {
            let formatted_bill_id = format!("{:05}", bill_id);
            for i in 0..3 {
                let res = self
                    .fetch_bill(HKBillWriterFetchBillRequest {
                        bill_id: formatted_bill_id.clone(),
                    })
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
            bill_id += 1;
            if bill_id > end {
                break;
            }
        }

        if bill_ids.is_empty() {
            Ok(Default::default())
        } else {
            Err(Error::FetchError(bill_ids))
        }
    }

    async fn fetch_recent_bills(&self) -> Result<HKBillWriter> {
        let HKBillWriter { bill_id, .. } = HKBillWriter::query_builder()
            .order_by_id_desc()
            .query()
            .map(HKBillWriter::from)
            .fetch_one(&self.pool)
            .await?;
        let mut bill = HKBillWriter::default();
        tracing::info!("last fetched bill: {:?}", bill_id);

        let mut bill_id = bill_id.parse::<i64>().unwrap_or(1);

        bill_id += 1;

        loop {
            let formatted_bill_id = format!("{:05}", bill_id);

            let res = self
                .fetch_bill(HKBillWriterFetchBillRequest {
                    bill_id: formatted_bill_id.clone(),
                })
                .await;
            tracing::debug!("fetched {:?}", res);

            if let Ok(b) = res {
                tracing::info!("fetched {}", bill_id);
                bill = b.clone();
            } else {
                tracing::debug!("no bill has been found");
            }

            bill_id += 1;

            if bill_id > MAX_BILL_ID {
                break;
            }
        }

        Ok(bill)
    }

    async fn fetch_bill(
        &self,
        HKBillWriterFetchBillRequest { bill_id }: HKBillWriterFetchBillRequest,
    ) -> Result<HKBillWriter> {
        tracing::debug!("fetch_bill {:?}", bill_id);

        let bill = self.cli.get_bill(bill_id.clone()).await?;

        let bill: HKBillWriter = bill.into();

        let bill = if let Some(b) = HKBillWriter::query_builder()
            .bill_id_equals(bill_id)
            .query()
            .map(HKBillWriter::from)
            .fetch_optional(&self.pool)
            .await?
        {
            let HKBillWriter {
                bill_id,
                year,
                bill_no,
                title,
                proposer,
                content_url,
                committee_name,
                proposed_date,
                first_reading_date,
                second_reading_date,
                third_reading_date,
                ordinance_date,
                additional_information,
                remarks,
                status,
                ..
            } = bill;
            self.repo
                .update(
                    b.id,
                    HKBillWriterRepositoryUpdateRequest {
                        bill_id: Some(bill_id),
                        year: Some(year),
                        bill_no: Some(bill_no),
                        title: Some(title),
                        proposer: Some(proposer),
                        content_url,
                        committee_name,
                        proposed_date: Some(proposed_date),
                        first_reading_date,
                        second_reading_date,
                        third_reading_date,
                        ordinance_date,
                        additional_information,
                        remarks,
                        status: Some(status),
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
                    bill.proposer,
                    bill.content_url,
                    bill.committee_name,
                    bill.proposed_date,
                    bill.first_reading_date,
                    bill.second_reading_date,
                    bill.third_reading_date,
                    bill.ordinance_date,
                    bill.additional_information,
                    bill.remarks,
                    bill.status,
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

        let cli = HKBillWriter::get_client(&endpoint);

        let bill = cli.fetch_bill(1).await;

        assert!(bill.is_ok(), "Failed to fetch bill: {:?}", bill);
        let bill = bill.unwrap();

        assert_eq!(bill.bill_id, "00001");
        assert_eq!(bill.title, "Adoption (Amendment) Bill 2003");
        assert_eq!(bill.year, 2004);
        assert_eq!(bill.bill_no, 28);
    }
}
