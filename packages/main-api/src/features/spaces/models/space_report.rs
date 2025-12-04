use crate::types::*;
use crate::utils::time::get_now_timestamp_millis;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default, JsonSchema)]
pub struct SpaceReport {
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,
    pub updated_at: i64,

    // Report content
    pub title: String,
    pub content: String,
    pub summary: Option<String>,
    pub files: Option<Vec<File>>,

    // Pricing (x402) - USD-based stablecoins
    pub price_dollars: Option<i64>,
    pub recipient_address: Option<String>,
    pub address_verified: bool,

    // Revenue split percentages
    pub treasury_percent: u8,
    pub platform_percent: u8,
    pub creator_percent: u8,

    // Publishing state
    pub publish_state: ReportPublishState,
    pub published_at: Option<i64>,

    // Author info
    pub author_pk: Partition,
    pub author_display_name: String,
    pub author_username: String,

    // GSI for listing published reports
    #[dynamo(prefix = "REPORT_PUBLISHED", name = "find_published", index = "gsi1", pk)]
    pub gsi1_pk: Option<String>,
    #[dynamo(index = "gsi1", sk)]
    pub gsi1_sk: Option<String>,
}

impl SpaceReport {
    pub fn new(space_pk: Partition, author_pk: Partition, author_display_name: String, author_username: String) -> Self {
        let now = get_now_timestamp_millis();
        Self {
            pk: space_pk,
            sk: EntityType::SpaceReport,
            created_at: now,
            updated_at: now,
            title: String::new(),
            content: String::new(),
            summary: None,
            files: None,
            price_dollars: None,
            recipient_address: None,
            address_verified: false,
            treasury_percent: 20,
            platform_percent: 10,
            creator_percent: 70,
            publish_state: ReportPublishState::Draft,
            published_at: None,
            author_pk,
            author_display_name,
            author_username,
            gsi1_pk: None,
            gsi1_sk: None,
        }
    }

    pub fn set_report_content(mut self, title: String, content: String, summary: Option<String>) -> Self {
        self.title = title;
        self.content = content;
        self.summary = summary;
        self.updated_at = get_now_timestamp_millis();
        self
    }

    pub fn with_pricing(mut self, price_dollars: i64, recipient_address: String) -> Self {
        self.price_dollars = Some(price_dollars);
        self.recipient_address = Some(recipient_address);
        self.address_verified = true;
        self.publish_state = ReportPublishState::PricingSet;
        self.updated_at = get_now_timestamp_millis();
        self
    }

    pub fn publish(mut self) -> Self {
        let now = get_now_timestamp_millis();
        self.publish_state = ReportPublishState::Published;
        self.published_at = Some(now);
        self.gsi1_pk = Some("REPORT_PUBLISHED".to_string());
        self.gsi1_sk = Some(now.to_string());
        self.updated_at = now;
        self
    }

    pub fn is_published(&self) -> bool {
        self.publish_state == ReportPublishState::Published
    }

    pub fn is_pricing_set(&self) -> bool {
        self.publish_state == ReportPublishState::PricingSet
    }

    pub fn can_publish(&self) -> bool {
        self.publish_state == ReportPublishState::PricingSet
            && self.price_dollars.is_some()
            && self.recipient_address.is_some()
            && self.address_verified
    }

    pub fn calculate_revenue_split(&self) -> Option<(i64, i64, i64)> {
        self.price_dollars.map(|price| {
            let treasury = price * self.treasury_percent as i64 / 100;
            let platform = price * self.platform_percent as i64 / 100;
            let creator = price - treasury - platform;
            (treasury, platform, creator)
        })
    }
}
