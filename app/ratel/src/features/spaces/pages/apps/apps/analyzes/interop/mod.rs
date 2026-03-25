use crate::features::spaces::pages::apps::apps::analyzes::*;

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

#[cfg(not(feature = "server"))]
use crate::common::wasm_bindgen::prelude::*;
#[cfg(not(feature = "server"))]
use crate::common::wasm_bindgen_futures::JsFuture;
#[cfg(not(feature = "server"))]
use crate::common::web_sys::js_sys::{Promise, Reflect, JSON};

#[cfg(not(feature = "server"))]
#[wasm_bindgen(js_namespace = ["window", "ratel", "spaces", "apps", "analyzes"])]
extern "C" {
    #[wasm_bindgen(js_name = downloadExcel, catch)]
    fn download_excel_js(req: &JsValue) -> std::result::Result<Promise, JsValue>;

    #[wasm_bindgen(js_name = renderBarChart, catch)]
    fn render_bar_chart_js(req: &JsValue) -> std::result::Result<(), JsValue>;

    #[wasm_bindgen(js_name = renderPieChart, catch)]
    fn render_pie_chart_js(req: &JsValue) -> std::result::Result<(), JsValue>;
}

#[cfg(not(feature = "server"))]
pub async fn download_analyze_excel(req: DownloadAnalyzeExcelRequest) -> Result<()> {
    let js_req = crate::common::serde_wasm_bindgen::to_value(&req)
        .map_err(|err| Error::Unknown(format!("failed to serialize excel request: {err}")))?;

    let promise = download_excel_js(&js_req).map_err(|err| Error::Unknown(format_js_error(err)))?;
    JsFuture::from(promise)
        .await
        .map_err(|err| Error::Unknown(format_js_error(err)))?;

    Ok(())
}

#[cfg(not(feature = "server"))]
pub fn render_analyze_bar_chart(req: &RenderAnalyzeBarChartRequest) -> Result<()> {
    let js_req = crate::common::serde_wasm_bindgen::to_value(req)
        .map_err(|err| Error::Unknown(format!("failed to serialize bar chart request: {err}")))?;

    render_bar_chart_js(&js_req).map_err(|err| Error::Unknown(format_js_error(err)))?;
    Ok(())
}

#[cfg(feature = "server")]
pub fn render_analyze_bar_chart(_req: &RenderAnalyzeBarChartRequest) -> Result<()> {
    Ok(())
}

#[cfg(not(feature = "server"))]
pub fn render_analyze_pie_chart(req: &RenderAnalyzePieChartRequest) -> Result<()> {
    let js_req = crate::common::serde_wasm_bindgen::to_value(req)
        .map_err(|err| Error::Unknown(format!("failed to serialize pie chart request: {err}")))?;

    render_pie_chart_js(&js_req).map_err(|err| Error::Unknown(format_js_error(err)))?;
    Ok(())
}

#[cfg(feature = "server")]
pub fn render_analyze_pie_chart(_req: &RenderAnalyzePieChartRequest) -> Result<()> {
    Ok(())
}

#[cfg(feature = "server")]
pub async fn download_analyze_excel(_req: DownloadAnalyzeExcelRequest) -> Result<()> {
    Err(Error::NotSupported(
        "excel download is only available on web".to_string(),
    ))
}

#[cfg(not(feature = "server"))]
fn format_js_error(err: JsValue) -> String {
    if let Some(message) = err.as_string() {
        return message;
    }

    if err.is_object() {
        if let Ok(message) = Reflect::get(&err, &JsValue::from_str("message")) {
            if let Some(message) = message.as_string() {
                return message;
            }
        }
    }

    if let Ok(json) = JSON::stringify(&err) {
        if let Some(message) = json.as_string() {
            return message;
        }
    }

    "Unknown error".to_string()
}
