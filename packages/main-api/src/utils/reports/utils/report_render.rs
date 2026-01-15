use crate::*;

use headless_chrome::types::PrintToPdfOptions;
use headless_chrome::{Browser, LaunchOptions, Tab};
use std::ffi::OsStr;
use std::time::{Duration, Instant};
use url::Url;

use super::report_html::build_report_html_document;

pub async fn render_report_pdf_bytes(html_contents: String) -> Result<Vec<u8>> {
    let bytes = tokio::task::spawn_blocking(move || -> Result<Vec<u8>> {
        let html_doc = build_report_html_document(&html_contents);

        let tmp =
            tempfile::tempdir().map_err(|e| crate::Error::InternalServerError(e.to_string()))?;
        let html_path = tmp.path().join("report.html");
        std::fs::write(&html_path, html_doc.as_bytes())
            .map_err(|e| crate::Error::InternalServerError(e.to_string()))?;

        let file_url = Url::from_file_path(&html_path)
            .map_err(|_| crate::Error::InternalServerError("failed to build file url".into()))?
            .to_string();

        let browser = Browser::new(LaunchOptions {
            headless: true,
            idle_browser_timeout: Duration::from_secs(120),
            args: vec![
                OsStr::new("--no-sandbox"),
                OsStr::new("--disable-dev-shm-usage"),
                OsStr::new("--disable-gpu"),
            ],
            ..Default::default()
        })
        .map_err(|e| crate::Error::InternalServerError(e.to_string()))?;

        let tab = browser
            .new_tab()
            .map_err(|e| crate::Error::InternalServerError(e.to_string()))?;

        tab.navigate_to(&file_url)
            .map_err(|e| crate::Error::InternalServerError(e.to_string()))?;

        wait_for_js_bool(
            &tab,
            "document.readyState === 'complete'",
            Duration::from_secs(20),
        )?;
        wait_for_js_bool(
            &tab,
            "window.__REPORT_RENDER_DONE__ === true",
            Duration::from_secs(60),
        )?;

        let pdf = tab
            .print_to_pdf(Some(PrintToPdfOptions {
                print_background: Some(true),
                prefer_css_page_size: Some(true),
                margin_top: Some(0.4),
                margin_bottom: Some(0.4),
                margin_left: Some(0.35),
                margin_right: Some(0.35),
                scale: Some(1.0),
                ..Default::default()
            }))
            .map_err(|e| crate::Error::InternalServerError(e.to_string()))?;

        Ok(pdf)
    })
    .await
    .map_err(|e| crate::Error::InternalServerError(format!("spawn_blocking failed: {e}")))??;

    Ok(bytes)
}

fn eval_string(tab: &Tab, expr: &str) -> String {
    tab.evaluate(expr, false)
        .ok()
        .and_then(|r| r.value)
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_else(|| "null".into())
}

fn eval_bool(tab: &Tab, expr: &str) -> bool {
    tab.evaluate(expr, false)
        .ok()
        .and_then(|r| r.value)
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
}

fn wait_for_js_bool(tab: &Tab, expr: &str, timeout: Duration) -> Result<()> {
    let start = Instant::now();
    loop {
        if eval_bool(tab, expr) {
            return Ok(());
        }

        if start.elapsed() > timeout {
            let ready = eval_string(tab, "document.readyState");
            let booted = eval_bool(tab, "window.__REPORT_BOOTED__ === true");
            let stage = eval_string(tab, "window.__REPORT_STAGE__ || ''");
            let err = eval_string(tab, "window.__REPORT_ERROR__ || ''");

            return Err(crate::Error::InternalServerError(format!(
                "render wait timeout (expr={expr}) readyState={ready} booted={booted} stage={stage} error={err}"
            )));
        }

        std::thread::sleep(Duration::from_millis(200));
    }
}
