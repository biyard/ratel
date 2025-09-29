#[macro_export]
macro_rules! post_with_body {
    (
        app: $app:expr,
        path: $path:expr,
        body: { $($body:tt)* },
        response_type: $resp_ty:ty $(,)?
    ) => {{
        use bdk::prelude::by_axum::axum;

        let req = axum::http::Request::builder()
            .uri(concat!("http://localhost:3000", $path))
            .method("POST")
            .header("content-type", "application/json")
            .body(axum::body::Body::from(
                serde_json::to_vec(&serde_json::json!({ $($body)* })).unwrap()
            ))
            .unwrap();

        let res: axum::http::Response<axum::body::Body> =
            tower::ServiceExt::oneshot($app.clone(), req).await.unwrap();

        let (parts, body) = res.into_parts();
        let body_bytes = axum::body::to_bytes(body, 10 * 1024 * 1024).await.unwrap();

        let parsed: $resp_ty = serde_json::from_slice(&body_bytes).unwrap();

        (parts.status, parts.headers, parsed)
    }};
}
