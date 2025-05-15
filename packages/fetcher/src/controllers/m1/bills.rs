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

use crate::modules::national_assembly::{AssemblyClient, html_parser::HtmlParser};

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct BillPath {
    pub id: i64,
}

#[derive(Clone, Debug)]
pub struct BillWriterController {
    repo: BillWriterRepository,
    cli: AssemblyClient,
    bill_channel: &'static str,
    pool: sqlx::Pool<sqlx::Postgres>,
}
static INSTANCE: OnceLock<bool> = OnceLock::new();

impl BillWriterController {
    pub async fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = BillWriter::get_repository(pool.clone());
        let cli = AssemblyClient::new(crate::config::get().openapi_key.to_string());
        let bill_channel = crate::config::get().slack.bill;

        let ctrl = Self {
            repo,
            pool,
            cli,
            bill_channel,
        };

        let arc = Arc::new(ctrl.clone());

        if INSTANCE.get().is_none() {
            let res = INSTANCE.set(true);
            if let Err(e) = res {
                btracing::notify_error!(
                    ctrl.bill_channel,
                    &format!("Failed to initialize INSTANCE on {e:?}",)
                );
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
        State(ctrl): State<BillWriterController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Json(body): Json<BillWriterAction>,
    ) -> Result<Json<BillWriter>> {
        tracing::debug!("act_bill {:?}", body);
        let res = match body {
            BillWriterAction::FetchBills(param) => {
                ctrl.fetch_bills(param.start_bill_no, param.end_bill_no)
                    .await?
            }
            BillWriterAction::FetchRecentBills(_) => ctrl.fetch_recent_bills().await?,
            BillWriterAction::FetchBill(param) => ctrl.fetch_bill(param).await?,
            BillWriterAction::FetchProposers(param) => ctrl.fetch_proposers(param.bill_no).await?,
        };
        Ok(Json(res))
    }
}

impl BillWriterController {
    async fn fetch_proposers(&self, bill_no: i64) -> Result<BillWriter> {
        let bill_info = self.cli.get_proposers(bill_no).await?;
        let bill_id = BillWriter::query_builder()
            .bill_no_equals(bill_no)
            .query()
            .map(BillWriter::from)
            .fetch_one(&self.pool)
            .await?
            .id;

        let rp = bill_info.get_representative_proposers();

        let representative_members =
            sqlx::query("SELECT * from assembly_members where name = ANY($1)")
                .bind(&rp)
                .map(AssemblyMember::from)
                .fetch_all(&self.pool)
                .await?;

        let cp = bill_info.get_co_proposers();
        let co_proposer_members =
            sqlx::query("SELECT * from assembly_members where name = ANY($1)")
                .bind(cp)
                .map(AssemblyMember::from)
                .fetch_all(&self.pool)
                .await?;

        let repo = Proposer::get_repository(self.pool.clone());
        let mut tx = self.pool.begin().await?;

        for p in representative_members {
            repo.insert_with_tx(&mut *tx, p.id, bill_id, true).await?;
        }

        for p in co_proposer_members {
            repo.insert_with_tx(&mut *tx, p.id, bill_id, false).await?;
        }

        tx.commit().await?;

        Ok(Default::default())
    }

    async fn fetch_bills(&self, mut bill_no: i64, end: i64) -> Result<BillWriter> {
        let mut bill_nos: Vec<(i64, String)> = vec![];

        loop {
            for i in 0..3 {
                let res = self
                    .fetch_bill(BillWriterFetchBillRequest { bill_no })
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
            Err(Error::FetchError(bill_nos))
        }
    }

    async fn fetch_recent_bills(&self) -> Result<BillWriter> {
        let BillWriter { mut bill_no, .. } = BillWriter::query_builder()
            .order_by_bill_no_desc()
            .query()
            .map(BillWriter::from)
            .fetch_one(&self.pool)
            .await?;
        let mut bill = BillWriter::default();
        tracing::info!("last fetched bill: {:?}", bill_no);
        bill_no += 1;

        loop {
            let res = self
                .fetch_bill(BillWriterFetchBillRequest { bill_no })
                .await;
            tracing::debug!("fetched {:?}", res);

            if let Ok(b) = res {
                tracing::info!("fetched {}", bill_no);
                bill = b.clone();
                btracing::notify!(
                    self.bill_channel,
                    &format!(
                        r#"
[{}] {}
{}
{}
{}
"#,
                        b.committee_name.unwrap_or_default(),
                        b.title,
                        b.proposer_name,
                        b.summary.unwrap_or_default(),
                        b.link_url
                    )
                );
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
        BillWriterFetchBillRequest { bill_no }: BillWriterFetchBillRequest,
    ) -> Result<BillWriter> {
        tracing::debug!("fetch_bill {}", bill_no);

        let bill = self.cli.get_bill(bill_no).await?;
        tracing::debug!("bill: {:?}", bill);

        let link_url = bill.link_url.clone();
        let mut bill: BillWriter = bill.into();

        {
            let parser = HtmlParser::new(&link_url).await?;
            let file_book_id = parser.get_file_book_id().unwrap_or_default();
            bill.book_id = file_book_id;

            if let Ok(description) = parser.get_description() {
                bill.summary = Some(description);
            } else {
                tracing::warn!("Failed to get description {}", link_url);
            };
        }

        let bill = if let Some(b) = BillWriter::query_builder()
            .bill_no_equals(bill_no)
            .query()
            .map(BillWriter::from)
            .fetch_optional(&self.pool)
            .await?
        {
            let BillWriter {
                bill_no,
                bill_id,
                title,
                book_id,
                date,
                en_title,
                summary,
                en_summary,
                proposer_kind,
                proposer_name,
                proposal_session,
                proposal_date,
                committee_name,
                committee_referral_date,
                committee_presentation_date,
                committee_processing_date,
                committee_processing_result,
                committee,
                law_committee_referral_date,
                law_committee_presentation_date,
                law_committee_processing_date,
                law_committee_processing_result,
                plenary_presentation_date,
                plenary_resolution_date,
                plenary_conference_name,
                plenary_conference_result,
                government_transfer_date,
                promulgated_law_name,
                promulgation_date,
                promulgation_number,
                link_url,
                ..
            } = bill;
            self.repo
                .update(
                    b.id,
                    BillWriterRepositoryUpdateRequest {
                        bill_no: Some(bill_no),
                        bill_id: Some(bill_id),
                        title: Some(title),
                        book_id: Some(book_id),
                        date: Some(date),
                        en_title,
                        summary,
                        en_summary,
                        proposer_kind: Some(proposer_kind),
                        proposer_name: Some(proposer_name),
                        proposal_session,
                        proposal_date: Some(proposal_date),
                        committee_name,
                        committee_referral_date,
                        committee_presentation_date,
                        committee_processing_date,
                        committee_processing_result,
                        committee,
                        law_committee_referral_date,
                        law_committee_presentation_date,
                        law_committee_processing_date,
                        law_committee_processing_result,
                        plenary_presentation_date,
                        plenary_resolution_date,
                        plenary_conference_name,
                        plenary_conference_result,
                        government_transfer_date,
                        promulgated_law_name,
                        promulgation_date,
                        promulgation_number,
                        link_url: Some(link_url),
                        industry: None,
                    },
                )
                .await?
        } else {
            self.repo
                .insert(
                    bill.bill_no,
                    bill.bill_id,
                    bill.title,
                    bill.book_id,
                    bill.date,
                    bill.en_title,
                    bill.summary,
                    bill.en_summary,
                    bill.proposer_kind,
                    bill.proposer_name,
                    bill.proposal_session,
                    bill.proposal_date,
                    bill.committee_name,
                    bill.committee_referral_date,
                    bill.committee_presentation_date,
                    bill.committee_processing_date,
                    bill.committee_processing_result,
                    bill.committee,
                    bill.law_committee_referral_date,
                    bill.law_committee_presentation_date,
                    bill.law_committee_processing_date,
                    bill.law_committee_processing_result,
                    bill.plenary_presentation_date,
                    bill.plenary_resolution_date,
                    bill.plenary_conference_name,
                    bill.plenary_conference_result,
                    bill.government_transfer_date,
                    bill.promulgated_law_name,
                    bill.promulgation_date,
                    bill.promulgation_number,
                    bill.link_url,
                    None,
                )
                .await?
        };

        if let Err(e) = self.fetch_proposers(bill.bill_no).await {
            tracing::warn!("Failed to fetch proposers: {:?}", e);
        }

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

        let cli = BillWriter::get_client(&endpoint);

        let bill = cli.fetch_bill(2200001).await;

        assert!(bill.is_ok(), "Failed to fetch bill: {:?}", bill);
        let bill = bill.unwrap();

        assert_eq!(bill.bill_no, 2200001);
        assert_eq!(bill.title, "교통약자의 이동편의 증진법 전부개정법률안");
        assert_eq!(bill.book_id, "42F04728-0F05-31CC-E17E-EB7EE4AEC7B8");
        assert_eq!(
            bill.summary,
            Some(
                "\n\t\t\t\t\t\t\t제안이유 \n \n  ‘이동권’은 모든 국민에게 기본권으로 보장되어야 할 권리임에도 불구하고 정부와 정치권이 그 책임을 다하지 않아 지하철 시위를 비롯한 장애인단체의 요구가 지속되고 있음. 게다가 사회적 갈등 양상으로 표출되고 정치와 일부 언론을 중심으로 장애인에 대한 혐오와 양극화는 심화되고 있음.  \n  이에 장애인단체의 지하철 시위를 멈추고, 사회적 갈등을 종식시키기 위한 교두보로 「교통약자의 이동편의 증진법」의 전부개정을 제안함.  \n  현행 「교통약자의 이동편의 증진법」(이하 ‘교통약자법’)은 인간으로서의 존엄과 가치 및 행복을 추구할 권리 보장을 위해 교통수단, 여객시설, 도로 등에 있어 이동권이 보장되어야 함을 명시하고 있음에도 불구하고 제한적인 이동편의시설과 서비스를 규정하고 있음. 이에 따라 교통약자들에게는 여전히 이동에 대한 물리적, 사회적 차별이 해소되지 않음.  \n  특히 이동권은 헌법에 명시된 자유권 그 자체이자 사회권 보장을 위한 전제임에도 불구하고 법령명에는 ‘편의’라는 용어가 사용되어 권리로서의 이동권의 의미를 퇴색시킴. 이에 교통약자법의 명칭을 ‘교통약자 이동권 보장을 위한 법률’로 변경하고 비장애인이 이용하고 있는 모든 교통수단과 여객시설 및 도로 등에 대해 교통약자가 차별받지 않고 시민으로서 그 권리를 보장받을 수 있도록 법률을 전면 개정하고자 함. \n \n \n주요내용 \n \n가. 교통약자 이동권 보장을 위한 주요 개념 추가 및 변경(안 제2조) \n  1) 교통수단 등에 택시, 광역철도 등을 추가하여 교통약자도 비교통약자와 동일하게 모든 환경에서 이동권을 보장받도록 하고자 함. \n  2) 이동편의서비스 개념을 명시하며 교통약자가 교통수단 및 여객시설 이용 시 지원을 받을 수 있도록 근거를 마련하고자 함. \n  3) 교통약자 이동지원차량의 정의를 구체화하여 특성에 따라 관련 서비스를 지원받을 수 있게 하고자 함.  \n나. 국가 교통약자 이동편의 증진계획 체계화 및 내용 구체화(안 제7조 등) \n  1) 혼재되어 있는 지원계획의 수립 주체를 국가, 광역, 기초 단위로 구분하여 명확히 정리하고자 함. \n  2) 계획에 시각장애인 등 다양한 특성의 교통약자와 미래 교통수단에 대한 내용을 포함하여 정책 수립 시 사각지대가 없도록 하고자 함. \n  3) 계획 수립 과정에서 교통약자 이동권 관련 단체가 참여하여 당사자ㆍ이용자 중심의 실효성 있는 방안이 검토될 수 있게 하고자 함. \n다. 교통약자 이동권 보장을 위한 전달체계 마련(안 제11조 등) \n  1) 교통약자 이동권 보장을 위한 국가 및 지방자치단체의 역할을 명시하고 이를 집행할 수 있는 기관을 체계적 형태로 설치하고자 함. \n  2) 국가, 광역, 기초 단위의 교통약자지원센터가 교통약자 이동지원차량 뿐 아니라 이동과 관련된 전반 업무를 관장하여 공백을 최소화하고 책임소재를 명확하게 하고자 함. \n라. 이동편의시설ㆍ서비스 관련 기준 추가 및 인증 체계화(안 제15조 등) \n  1) 이동편의시설 및 서비스 제공 시 발달장애인 등 사각지대의 교통약자를 고려해 읽기 쉬운 표지, 시ㆍ청각서비스 등을 추가하고 국토교통부가 관련 서비스의 전면 보급을 위해 교재 등을 개발하게 함. \n  2) 시설 및 서비스의 원활한 수행을 위한 기관을 운영 및 지정하게 함. \n마. 교통사업자 및 승무원 교육(안 제26조 등) \n  1) 교육을 의무적으로 이수해야 하는 범위를 도시철도, 궤도운송, 해운까지 확대하여 서비스 제공에 공백이 발생하지 않게 함.  \n  2) 교통사업자 및 승무원 교육 시 서비스 뿐 아니라 교통약자의 인권 및 특성 관련 내용을 추가하여 교통약자에 대해 이해시키고자 함. \n  3) 국토교통부가 교육 여부 및 결과를 의무적으로 수합하고 이를 공개하여 취지에 따라 운영될 수 있도록 점검하고자 함. \n바. 버스 및 택시에 대한 교통약자의 이용 보장(안 제6장제1절) \n  1) 교통사업자 및 승무원이 교통약자의 버스 이용 시 관련 서비스를 차별 없이 제공할 수 있도록 승하차 지원 및 적재물 제거 등의 내용을 명시함. \n  2) 「여객자동차 운수사업법」에 따른 모든 버스(시외ㆍ고속버스 포함)에 휠체어 탑승설비가 설치된 차량을 의무 도입하고 유예 시 개선 및 대안마련 기간을 명시하여 휠체어 이용자의 접근권을 보장하고자 함. \n  3) 「택시운송사업의 발전에 관한 법률」에 따른 모든 택시에 휠체어 탑승설비가 설치된 차량을 의무화하고 관련 서비스를 적시하여 택시 이용 시 교통약자에 대한 차별이 발생하지 않게 하고자 함. \n사. 철도, 항공, 해운에 대한 교통약자의 이용 보장(안 제6장제2절) \n  1) 교통약자의 철도, 도시철도, 항공, 해운 이용을 위한 별도의 조항이 기존 법안에 마련되어 있지 않음에 따라 좌석 및 전용구역 확보, 승강장까지의 이동방안, 시각장애인을 위한 표지 설치 등 교통수단 내 편의시설 및 서비스 기준을 명시함.  \n  2) 해운, 항공 등 교통약자 이동권 보장을 위한 비용 필요 시 국토교통부에서 관련 자금을 지원할 수 있게 함.  \n아. 교통약자 이동지원차량의 종류별 목적 및 이용자의 특성을 고려한 전달체계의 역할과 응급의료 및 위급한 상황 시의 이동지원 명기(안 제38조) \n  1) 휠체어 이동지원차량, 이동서비스 지원차량, 단순이동 지원차량의 종류별 정의와 이용자를 구분함. \n  2) 교통약자 이동지원차량의 원활한 이용을 위해 등록 및 배차 등의 역할을 국가교통약자이동지원시스템으로 일원화하고, 국토교통부장관 및 지방자치단체의 의무를 명기함. \n  3) 응급의료 및 위급한 상황을 대비한 상시운행 및 지원체계를 만들고자 함. \n자. 개인형 이동수단 이용 및 관련 사업자에 대한 책임 명시(안 제49조) \n  1) 개인형 이동수단으로 인한 교통약자의 보행환경 제한을 해결하기 위해 이동수단 주차 시 점자보도 블록, 진로 등을 확보하고자 함. \n  2) 개인형 이동수단으로 인한 사고 및 상해 발생 시 교통약자 지원을 위해 사업자의 책임으로 명시하고자 함. \n차. 법률 미이행에 대한 제재 방안 정비(안 제57조 등) \n  1) 법 미이행 시 교통약자의 이동권이 제대로 보장되고 재발을 방지하기 위해 이동편의시설 및 서비스 설치ㆍ관리, 교통수단 이용 등 관련 상황과 내용을 구체적으로 정비하고자 함.\n\t\t\t\t\t\t".to_string()
            )
        );
    }
}
