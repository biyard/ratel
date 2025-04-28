use bdk::prelude::*;
use dto::{Result, ServiceError};
use scraper::{Html, Selector};

#[derive(Debug, Clone, PartialEq)]
pub struct HtmlParser {
    doc: Html,
}

impl HtmlParser {
    pub async fn new(site_link: &str) -> Result<Self> {
        let client = reqwest::Client::new();
        let resp = client
            .get(site_link)
            .header(reqwest::header::USER_AGENT, "biyard")
            .send()
            .await?
            .text()
            .await?;

        let doc = Html::parse_document(&resp);

        Ok(Self { doc })
    }

    pub fn get_file_book_id(&self) -> Result<String> {
        let selector = Selector::parse(r#"a[href^="javascript:openBillFile"]"#).unwrap();

        if let Some(element) = self.doc.select(&selector).next() {
            if let Some(href) = element.value().attr("href") {
                tracing::debug!("href: {}", href);
                let parts: Vec<&str> = href.split(',').collect();
                if let Some(book_id) = parts.get(1) {
                    let book_id = book_id.trim_matches(|c| c == '\'' || c == ' ' || c == '\u{a0}');
                    return Ok(book_id.to_string());
                }
            }
        }

        tracing::error!("Failed to find the element with the selector");
        Err(ServiceError::HtmlParseError(
            "Failed to parse response".to_string(),
        ))
    }

    pub fn get_description(&self) -> Result<String> {
        let selector = Selector::parse(r#"div[id="summaryContentDiv"]"#).unwrap();

        if let Some(element) = self.doc.select(&selector).next() {
            let description = element.text().collect::<Vec<_>>().join(" ");
            return Ok(description);
        } else {
            tracing::error!("Failed to find the element with the selector");
            Err(ServiceError::HtmlParseError(
                "Failed to parse response".to_string(),
            ))
        }
    }
}
