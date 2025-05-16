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

use crate::modules::eu_parliament::EuParliamentClient;

#[derive(Clone, Debug)]
pub struct EUBillWriterController {
    repo: EUBillWriterRepository,
    cli: EuParliamentClient,
    pool: sqlx::Pool<sqlx::Postgres>,
}

static INSTANCE: OnceLock<bool> = OnceLock::new();
static CURRENT_YEAR: i64 = chrono::Utc::now().year() as i64;
static CURRENT_TERM: i64 = 10;

impl EUBillWriterController {
    pub async fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = EUBillWriter::get_repository(pool.clone());
        let cli = EuParliamentClient::new();

        let ctrl = Self { repo, pool, cli };

        let arc = Arc::new(ctrl.clone());

        if INSTANCE.get().is_none() {
            let res = INSTANCE.set(true);
            if let Err(e) = res {
                tracing::error!("Failed to initialize INSTANCE on {e:?}");
            }
            tokio::spawn(async move {
                let _ = arc.fetch_recent_bills(CURRENT_YEAR, CURRENT_TERM).await;
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
        State(ctrl): State<EUBillWriterController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Json(body): Json<EUBillWriterAction>,
    ) -> Result<Json<EUBillWriter>> {
        tracing::debug!("act_bill {:?}", body);
        let res = match body {
            EUBillWriterAction::FetchBills(param) => {
                ctrl.fetch_bills(Some(param.year), param.start_bill_no, param.end_bill_no)
                    .await?
            }
            EUBillWriterAction::FetchRecentBills(param) => {
                ctrl.fetch_recent_bills(param.year, param.parliamentary_term)
                    .await?
            }
            EUBillWriterAction::FetchBill(param) => ctrl.fetch_bill(param).await?,
        };
        Ok(Json(res))
    }
}

impl EUBillWriterController {
    async fn fetch_bills(
        &self,
        year: Option<i64>,
        mut bill_no: i64,
        end: i64,
    ) -> Result<EUBillWriter> {
        tracing::debug!("fetch_bills {:?} {:?} {:?}", year, bill_no, end);
        let mut bill_ids: Vec<(String, String)> = vec![];

        loop {
            let bill_id_list = self.cli.list_bill_id(year, bill_no, 1).await?;
            tracing::debug!("bill_id_list {:?}", bill_id_list);

            if bill_id_list.is_empty() {
                tracing::info!("no more bills to fetch");
                break;
            }

            for i in 0..3 {
                let bill_id = bill_id_list[0].clone();
                let bill_id_clone = bill_id.clone();
                let res = self
                    .fetch_bill(EUBillWriterFetchBillRequest { bill_id })
                    .await;

                if res.is_ok() {
                    tracing::info!("fetched {} bill", bill_id_clone);
                    break;
                } else {
                    tracing::error!("Failed to fetch bill {}: {:?}", bill_id_clone.clone(), res);
                    if i == 2 {
                        bill_ids.push((bill_id_clone, format!("{:?}", res)));
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
            Err(Error::EuOpenDataFetchError(bill_ids))
        }
    }

    async fn fetch_recent_bills(&self, year: i64, term: i64) -> Result<EUBillWriter> {
        let EUBillWriter { mut bill_no, .. } = EUBillWriter::query_builder()
            .parliamentary_term_equals(term)
            .year_equals(year)
            .order_by_id_desc()
            .query()
            .map(EUBillWriter::from)
            .fetch_one(&self.pool)
            .await?;
        let mut bill = EUBillWriter::default();
        tracing::info!("last fetched bill: {:?}", bill_no);
        bill_no += 1;

        loop {
            let bill_id = format!("TA-{}-{:04}-{:04}", term, year, bill_no);
            let res = self
                .fetch_bill(EUBillWriterFetchBillRequest { bill_id })
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
        EUBillWriterFetchBillRequest { bill_id }: EUBillWriterFetchBillRequest,
    ) -> Result<EUBillWriter> {
        tracing::debug!("fetch_bill {:?}", bill_id);

        let bill = self.cli.get_bill(bill_id.clone()).await?;

        let bill: EUBillWriter = bill.into();

        let bill = if let Some(b) = EUBillWriter::query_builder()
            .bill_id_equals(bill_id)
            .query()
            .map(EUBillWriter::from)
            .fetch_optional(&self.pool)
            .await?
        {
            let EUBillWriter {
                bill_id,
                parliamentary_term,
                year,
                bill_no,
                date,
                label,
                ep_number,
                title,
                alternative_title,
                pdf_url,
                xml_url,
                docs_url,
                subject_matter,
                ..
            } = bill;
            self.repo
                .update(
                    b.id,
                    EUBillWriterRepositoryUpdateRequest {
                        bill_id: Some(bill_id),
                        parliamentary_term: Some(parliamentary_term),
                        year: Some(year),
                        bill_no: Some(bill_no),
                        date: Some(date),
                        label: Some(label),
                        ep_number: Some(ep_number),
                        title: Some(title),
                        alternative_title,
                        pdf_url,
                        xml_url,
                        docs_url,
                        subject_matter,
                    },
                )
                .await?
        } else {
            self.repo
                .insert(
                    bill.bill_id,
                    bill.year,
                    bill.parliamentary_term,
                    bill.bill_no,
                    bill.date,
                    bill.label,
                    bill.ep_number,
                    bill.title,
                    bill.alternative_title,
                    bill.pdf_url,
                    bill.xml_url,
                    bill.docs_url,
                    None, // TODO: parse many subject matters
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

        let cli = EUBillWriter::get_client(&endpoint);

        let bill = cli.fetch_bill("TA-9-2022-0201".to_string()).await;

        assert!(bill.is_ok(), "Failed to fetch bill: {:?}", bill);
        let bill = bill.unwrap();

        assert_eq!(bill.bill_id, "TA-9-2022-0201");
        assert_eq!(
            bill.title,
            "The continuous crackdown of political opposition in Cambodia ".to_string()
        );
        assert_eq!(bill.date, 20250505);
        assert_eq!(bill.parliamentary_term, 9);
        assert_eq!(bill.ep_number, "PE719.542");
        assert_eq!(bill.label, "T9-0201/2022");
    }
}
