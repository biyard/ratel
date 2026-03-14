use bdk::prelude::tracing;
use by_axum::axum::{
    body::Body,
    extract::Request,
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};

const UPSTREAM_URL: &str = env!("UPSTREAM_URL");

pub async fn reverse_proxy(req: Request<Body>) -> Response {
    let client = reqwest::Client::new();

    let uri = req.uri();
    let path_and_query = uri
        .path_and_query()
        .map(|pq| pq.as_str())
        .unwrap_or(uri.path());

    let upstream_url = format!("{}{}", UPSTREAM_URL, path_and_query);

    let method = req.method().clone();
    let headers = req.headers().clone();

    let body_bytes = match by_axum::axum::body::to_bytes(req.into_body(), usize::MAX).await {
        Ok(bytes) => bytes,
        Err(e) => {
            tracing::error!("Failed to read request body: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read request body",
            )
                .into_response();
        }
    };

    let mut upstream_req = client.request(
        reqwest::Method::from_bytes(method.as_str().as_bytes()).unwrap_or(reqwest::Method::GET),
        &upstream_url,
    );

    for (key, value) in headers.iter() {
        if key == header::HOST || key == header::CONNECTION || key == header::TRANSFER_ENCODING {
            continue;
        }
        if let Ok(value_str) = value.to_str() {
            upstream_req = upstream_req.header(key.as_str(), value_str);
        }
    }

    if !body_bytes.is_empty() {
        upstream_req = upstream_req.body(body_bytes.to_vec());
    }

    match upstream_req.send().await {
        Ok(upstream_res) => {
            let status = StatusCode::from_u16(upstream_res.status().as_u16())
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

            let mut response_builder = Response::builder().status(status);

            for (key, value) in upstream_res.headers().iter() {
                if key == header::TRANSFER_ENCODING || key == header::CONNECTION {
                    continue;
                }
                if let Ok(value_str) = value.to_str() {
                    response_builder = response_builder.header(key.as_str(), value_str);
                }
            }

            match upstream_res.bytes().await {
                Ok(body) => response_builder.body(Body::from(body)).unwrap_or_else(|_| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to build response",
                    )
                        .into_response()
                }),
                Err(e) => {
                    tracing::error!("Failed to read upstream response body: {}", e);
                    (
                        StatusCode::BAD_GATEWAY,
                        "Failed to read upstream response body",
                    )
                        .into_response()
                }
            }
        }
        Err(e) => {
            tracing::error!("Failed to proxy request to {}: {}", upstream_url, e);
            (StatusCode::BAD_GATEWAY, format!("Upstream error: {}", e)).into_response()
        }
    }
}
