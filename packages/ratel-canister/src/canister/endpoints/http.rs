use candid::CandidType;
use serde::Deserialize;

use super::system::build_version;

#[derive(CandidType, Deserialize)]
pub(crate) struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

#[derive(CandidType)]
pub(crate) struct HttpResponse {
    pub status_code: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

#[ic_cdk::query]
fn http_request(req: HttpRequest) -> HttpResponse {
    let path = req.url.split('?').next().unwrap_or(&req.url);

    match path {
        "/version" => HttpResponse {
            status_code: 200,
            headers: vec![("content-type".into(), "text/plain".into())],
            body: build_version().into_bytes(),
        },
        "/health" => HttpResponse {
            status_code: 200,
            headers: vec![("content-type".into(), "text/plain".into())],
            body: b"ok".to_vec(),
        },
        _ => HttpResponse {
            status_code: 404,
            headers: vec![("content-type".into(), "text/plain".into())],
            body: b"not found".to_vec(),
        },
    }
}
