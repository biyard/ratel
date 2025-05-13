use crate::models::openapi::national_bill::AssemblyBill;
use bdk::prelude::*;
use dto::Error;
use scraper::{Html, Selector};
use serde_json::Value;
use std::collections::HashMap;

const ASSEMBLY_UNIT: &str = "제22대";

pub async fn fetch_bills(
    page_index: u32, // page num; start from 1 not 0
    page_size: u32,  // request per page
) -> Result<Vec<AssemblyBill>, Error> {
    let config = crate::config::get();
    let mut params = HashMap::new();
    params.insert("KEY", config.openapi_key.to_string());
    params.insert("type", "json".to_string());
    params.insert("pIndex", page_index.to_string());
    params.insert("pSize", page_size.to_string());
    params.insert("ERACO", ASSEMBLY_UNIT.to_string());

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/BILLJUDGE", config.openapi_url))
        .query(&params)
        .header(reqwest::header::USER_AGENT, "biyard") // Required
        .send()
        .await?
        .text()
        .await?;

    if let Ok(json) = serde_json::from_str::<Value>(&response) {
        let response = json["BILLJUDGE"].clone();
        let rows = match response[1]["row"].as_array() {
            Some(rows) => rows,
            None => {
                return Err(Error::OpenApiResponseError(
                    "Failed to parse response".to_string(),
                ));
            }
        };
        let rst: Vec<AssemblyBill> =
            match serde_json::from_value(serde_json::Value::Array(rows.clone())) {
                Ok(rst) => rst,
                Err(e) => {
                    return Err(Error::JsonDeserializeError(e.to_string()));
                }
            };
        return Ok(rst);
    } else {
        return Err(Error::OpenApiResponseError(
            "Failed to parse response".to_string(),
        ));
    }
}

pub async fn get_file_book_id(site_link: String, // link to the file
) -> Result<String, Error> {
    let client = reqwest::Client::new();
    let resp = client
        .get(site_link)
        .header(reqwest::header::USER_AGENT, "biyard") //
        .send()
        .await?
        .text()
        .await?;

    // xpath: /html/body/div/div[2]/div[2]/div/div[3]/div[1]/table/tbody/tr/td[4]/a[1]
    // or .tableCol01 > a:nth-child(1)

    let doc = Html::parse_document(&resp);
    let selector = Selector::parse(r#"a[href^="javascript:openBillFile"]"#).unwrap();

    if let Some(element) = doc.select(&selector).next() {
        if let Some(href) = element.value().attr("href") {
            tracing::debug!("href: {}", href);
            let parts: Vec<&str> = href.split(',').collect();
            if let Some(book_id) = parts.get(1) {
                let book_id = book_id.trim_matches(|c| c == '\'' || c == ' ' || c == '\u{a0}');
                return Ok(book_id.to_string());
            }
        }
    }
    Err(Error::HtmlParseError(
        "Failed to parse response".to_string(),
    ))
}
