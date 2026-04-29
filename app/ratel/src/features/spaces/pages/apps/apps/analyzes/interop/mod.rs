//! JS interop for the Analyze app — Excel export + chart helpers.
//!
//! Mirrors `features/auth/interop/wallet_connect.rs`: each public fn
//! drives a tiny `web/<name>.js` script via `dioxus::document::eval`.
//! `dx_eval` is platform-agnostic (no-op outside web), so no
//! per-target cfg gates are needed at the call site.
//! Direct `wasm_bindgen(js_namespace = […]) extern "C"` mappings are
//! an anti-pattern — see `conventions/anti-patterns.md`.

use dioxus::document::eval as dx_eval;

use crate::features::spaces::pages::apps::apps::analyzes::*;
use crate::features::spaces::pages::apps::types::SpaceAppError;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DownloadAnalyzeExcelRequest {
    pub file_name: String,
    pub sheet_name: String,
    pub rows: Vec<Vec<String>>,
    pub merges: Vec<AnalyzeExcelMerge>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AnalyzeExcelMerge {
    pub start_row: usize,
    pub start_col: usize,
    pub end_row: usize,
    pub end_col: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AnalyzeChartDatum {
    pub label: String,
    pub count: i64,
    pub percentage: f64,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RenderAnalyzeBarChartRequest {
    pub container_id: String,
    pub entries: Vec<AnalyzeChartDatum>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RenderAnalyzePieChartRequest {
    pub container_id: String,
    pub entries: Vec<AnalyzeChartDatum>,
}

// ── Excel export ──────────────────────────────────────────────────

pub async fn download_analyze_excel(req: DownloadAnalyzeExcelRequest) -> Result<()> {
    let mut runner = dx_eval(include_str!("web/download_excel.js"));
    runner
        .send(serde_json::to_value(&req).map_err(|_| SpaceAppError::ExcelExportFailed)?)
        .map_err(|_| SpaceAppError::ExcelExportFailed)?;
    let ok = runner
        .recv::<Option<bool>>()
        .await
        .map_err(|_| SpaceAppError::ExcelExportFailed)?;
    match ok {
        Some(true) => Ok(()),
        _ => Err(SpaceAppError::ExcelExportFailed.into()),
    }
}

// ── Chart helpers (placeholders — call sites land with the chart UI) ─

pub async fn render_analyze_bar_chart(req: &RenderAnalyzeBarChartRequest) -> Result<()> {
    let mut runner = dx_eval(include_str!("web/render_bar_chart.js"));
    runner
        .send(serde_json::to_value(req).map_err(|_| SpaceAppError::ChartRenderFailed)?)
        .map_err(|_| SpaceAppError::ChartRenderFailed)?;
    let ok = runner
        .recv::<Option<bool>>()
        .await
        .map_err(|_| SpaceAppError::ChartRenderFailed)?;
    match ok {
        Some(true) => Ok(()),
        _ => Err(SpaceAppError::ChartRenderFailed.into()),
    }
}

pub async fn render_analyze_pie_chart(req: &RenderAnalyzePieChartRequest) -> Result<()> {
    let mut runner = dx_eval(include_str!("web/render_pie_chart.js"));
    runner
        .send(serde_json::to_value(req).map_err(|_| SpaceAppError::ChartRenderFailed)?)
        .map_err(|_| SpaceAppError::ChartRenderFailed)?;
    let ok = runner
        .recv::<Option<bool>>()
        .await
        .map_err(|_| SpaceAppError::ChartRenderFailed)?;
    match ok {
        Some(true) => Ok(()),
        _ => Err(SpaceAppError::ChartRenderFailed.into()),
    }
}
