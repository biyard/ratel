use std::{
    sync::{Arc, OnceLock},
    time::Duration,
};

use bdk::prelude::*;
use by_axum::{
    aide,
    auth::Authorization,
    axum::{Extension, Json, extract::State, native_routing::post},
};
use dto::*;
use tokio::time::sleep;

use crate::modules::us_congress::{UsCongressClient, convert_to_bill_writer};

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct BillPath {
    pub id: i64,
}

#[derive(Clone, Debug)]
pub struct USBillWriterController {
    repo: USBillWriterRepository,
    cli: UsCongressClient,
    // bill_channel: &'static str,
    pool: sqlx::Pool<sqlx::Postgres>,
}

static INSTANCE: OnceLock<bool> = OnceLock::new();
static CURRENT_CONGRESS: i64 = 119;

impl USBillWriterController {
    pub async fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = USBillWriter::get_repository(pool.clone());
        let cli = UsCongressClient::new(crate::config::get().us_congress_key.to_string());
        // let bill_channel = crate::config::get().slack.bill;

        let ctrl = Self {
            repo,
            pool,
            cli,
            // bill_channel,
        };

        let arc_hr = Arc::new(ctrl.clone());
        let arc_s = Arc::new(ctrl.clone());

        if INSTANCE.get().is_none() {
            let res = INSTANCE.set(true);
            if let Err(e) = res {
                // btracing::notify_error!(
                //     ctrl.bill_channel,
                //     &format!("Failed to initialize INSTANCE on {e:?}",)
                // );
                tracing::error!("Failed to initialize INSTANCE on {e:?}");
            }
            tokio::spawn(async move {
                let _ = arc_hr
                    .fetch_recent_bills(CURRENT_CONGRESS, dto::USBillType::HouseBill)
                    .await;
                let _ = arc_s
                    .fetch_recent_bills(CURRENT_CONGRESS, dto::USBillType::SenateBill)
                    .await;
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
        State(ctrl): State<USBillWriterController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Json(body): Json<USBillWriterAction>,
    ) -> Result<Json<USBillWriter>> {
        tracing::debug!("act_bill {:?}", body);
        let res = match body {
            USBillWriterAction::FetchBills(param) => {
                ctrl.fetch_bills(
                    param.congress,
                    param.bill_type,
                    param.start_bill_no,
                    param.end_bill_no,
                )
                .await?
            }
            USBillWriterAction::FetchRecentBills(param) => {
                ctrl.fetch_recent_bills(param.congress, param.bill_type)
                    .await?
            }
            USBillWriterAction::FetchBill(param) => ctrl.fetch_bill(param).await?,
        };
        Ok(Json(res))
    }
}

impl USBillWriterController {
    async fn fetch_bills(
        &self,
        congress: i64,
        bill_type: USBillType,
        mut bill_no: i64,
        end: i64,
    ) -> Result<USBillWriter> {
        let mut bill_nos: Vec<(i64, String)> = vec![];
        loop {
            for i in 0..3 {
                let res = self
                    .fetch_bill(USBillWriterFetchBillRequest {
                        congress,
                        bill_no,
                        bill_type,
                    })
                    .await;

                if res.is_ok() {
                    tracing::info!("fetched {} bill", bill_no);
                    break;
                } else {
                    tracing::error!("Failed to fetch bill {}: {:?}", bill_no, res);
                    if i == 2 {
                        bill_nos.push((bill_no, format!("{:?}", res)));
                    }
                }
            }
            bill_no += 1;
            if bill_no > end {
                break;
            }
        }

        if bill_nos.is_empty() {
            Ok(Default::default())
        } else {
            Err(ServiceError::FetchError(bill_nos))
        }
    }

    async fn fetch_recent_bills(
        &self,
        congress: i64,
        bill_type: USBillType,
    ) -> Result<USBillWriter> {
        let USBillWriter { mut bill_no, .. } = USBillWriter::query_builder()
            .congress_equals(congress)
            .bill_type_equals(bill_type)
            .order_by_bill_no_desc()
            .query()
            .map(USBillWriter::from)
            .fetch_one(&self.pool)
            .await?;
        let mut bill = USBillWriter::default();
        tracing::info!("last fetched bill: {:?}", bill_no);
        bill_no += 1;

        loop {
            let res = self
                .fetch_bill(USBillWriterFetchBillRequest {
                    congress,
                    bill_type,
                    bill_no,
                })
                .await;
            tracing::debug!("fetched {:?}", res);

            if let Ok(b) = res {
                tracing::info!("fetched {}", bill_no);
                bill = b.clone();
                //                 btracing::notify!(
                //                     self.bill_channel,
                //                     &format!(
                //                         r#"
                // [{}] {}
                // {}
                // {}
                // {}
                // "#,
                //                         b.committee_name.unwrap_or_default(),
                //                         b.title,
                //                         b.proposer_name,
                //                         b.summary.unwrap_or_default(),
                //                         b.link_url
                //                     )
                //                 );
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
        USBillWriterFetchBillRequest {
            congress,
            bill_type,
            bill_no,
        }: USBillWriterFetchBillRequest,
    ) -> Result<USBillWriter> {
        tracing::debug!("fetch_bill {:?} {:?} {:?}", congress, bill_type, bill_no);

        let bill = self
            .cli
            .get_bill(congress, bill_type.to_code(), bill_no)
            .await?;

        let titles = self
            .cli
            .get_bill_titles(congress, bill_type.to_code(), bill_no)
            .await?;

        let texts = self
            .cli
            .get_bill_text(congress, bill_type.to_code(), bill_no)
            .await?;

        let summary = self
            .cli
            .get_bill_summary(congress, bill_type.to_code(), bill_no)
            .await?;

        let subject = self
            .cli
            .get_bill_subject(congress, bill_type.to_code(), bill_no)
            .await?;

        let bill: USBillWriter = convert_to_bill_writer(bill, titles, subject, summary, texts);

        let bill = if let Some(b) = USBillWriter::query_builder()
            .congress_equals(congress)
            .bill_type_equals(bill_type)
            .bill_no_equals(bill_no)
            .query()
            .map(USBillWriter::from)
            .fetch_optional(&self.pool)
            .await?
        {
            let USBillWriter {
                congress,
                bill_type,
                bill_no,
                bill_id,
                title,
                summary,

                html_url,
                pdf_url,
                xml_url,
                origin_chamber,
                action_date,
                update_date,
                industry,
                ..
            } = bill;
            self.repo
                .update(
                    b.id,
                    USBillWriterRepositoryUpdateRequest {
                        congress: Some(congress),
                        bill_type: Some(bill_type),
                        bill_no: Some(bill_no),
                        bill_id: Some(bill_id),
                        title: Some(title),
                        summary: Some(summary),

                        html_url: html_url,
                        pdf_url: pdf_url,
                        xml_url: xml_url,
                        origin_chamber: Some(origin_chamber),
                        action_date: Some(action_date),
                        update_date: Some(update_date),
                        industry: Some(industry),
                    },
                )
                .await?
        } else {
            self.repo
                .insert(
                    bill.congress,
                    bill.bill_type,
                    bill.bill_no,
                    bill.title,
                    bill.summary,
                    bill.bill_id,
                    bill.html_url,
                    bill.pdf_url,
                    bill.xml_url,
                    bill.origin_chamber,
                    bill.action_date,
                    bill.update_date,
                    bill.industry,
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

        let cli = USBillWriter::get_client(&endpoint);

        let bill = cli.fetch_bill(117, dto::USBillType::HouseBill, 3076).await;

        assert!(bill.is_ok(), "Failed to fetch bill: {:?}", bill);
        let bill = bill.unwrap();

        assert_eq!(bill.bill_id, "hr3076-117");
        assert_eq!(bill.title, "Postal Service Reform Act of 2022");
        assert_eq!(
            bill.summary,
            " <p><strong>Postal Service Reform Act of 202</strong><strong>2</strong></p> <p>This bill addresses the finances and operations of the U.S. Postal Service (USPS).</p> <p>The bill requires the Office of Personnel Management (OPM) to establish the Postal Service Health Benefits Program within the Federal Employees Health Benefits Program under which OPM may contract with carriers to offer health benefits plans for USPS employees and retirees.</p> <p>The bill provides for coordinated enrollment of retirees under this program and Medicare.</p> <p>The bill repeals the requirement that the USPS annually prepay future retirement health benefits.</p> <p>Additionally, the USPS may establish a program to enter into agreements with an agency of any state government, local government, or tribal government, and with other government agencies, to provide certain nonpostal products and services that reasonably contribute to the costs of the USPS and meet other specified criteria.</p> <p>The USPS must develop and maintain a publicly available dashboard to track service performance and must report regularly on its operations and financial condition.</p> <p>The Postal Regulatory Commission must annually submit to the USPS a budget of its expenses. It must also conduct a study to identify the causes and effects of postal inefficiencies relating to flats (e.g., large envelopes).</p> <p>The USPS Office of Inspector General shall perform oversight of the Postal Regulatory Commission. </p>".to_string(),  
        );
    }
}
