#[macro_export]
macro_rules! call {
    (
        app: $app:expr,
        path: $path:expr,
        method: $method:expr,
        body:  $body:expr
    ) => {{
        $crate::call! {
            app: $app,
            path: $path,
            method: $method,
            body: $body,
            headers: axum::http::HeaderMap::new()
        }
    }};

    (
        app: $app:expr,
        path: $path:expr,
        method: $method:expr,
        body:  $body:expr,
        headers: $headers:expr
    ) => {{
        use axum::http::header::{self, HeaderValue};
        use bdk::prelude::by_axum::axum;

        let mut req_builder = axum::http::Request::builder()
            .uri(concat!("http://localhost:3000", $path))
            .method($method);

        if let Some(headers_mut) = req_builder.headers_mut() {
            headers_mut.extend($headers);
            headers_mut
                .entry(header::CONTENT_TYPE)
                .or_insert(HeaderValue::from_static("application/json"));
        }

        let req = req_builder.body($body).unwrap();

        let res: axum::http::Response<axum::body::Body> =
            tower::ServiceExt::oneshot($app.clone(), req).await.unwrap();

        let (parts, body) = res.into_parts();
        let body_bytes = axum::body::to_bytes(body, 10 * 1024 * 1024)
            .await
            .unwrap()
            .to_vec();

        (
            parts.status,
            parts.headers,
            String::from_utf8(body_bytes).unwrap(),
        )
    }};
}

#[macro_export]
macro_rules! send {
    (
        app: $app:expr,
        method: $method:expr,
        path: $path:expr,
        headers: $headers:expr,
        body: { $($body:tt)* },
        response_type: $resp_ty:ty
    ) => {{
        use bdk::prelude::by_axum::axum;
        let body = axum::body::Body::from(serde_json::to_vec(&serde_json::json!({ $($body)* })).unwrap());
        let (status, headers, parsed) = $crate::call! { app: $app, path: $path, method: $method, body: body, headers: $headers };
        let parsed = serde_json::from_str::<$resp_ty>(&parsed).unwrap();
        (status, headers, parsed)
    }};

    (
        app: $app:expr,
        method: $method:expr,
        path: $path:expr,
        body: { $($body:tt)* },
        response_type: $resp_ty:ty
    ) => {{
        $crate::send! { app: $app, method: $method, path: $path, headers: axum::http::HeaderMap::new(), body: { $($body)* }, response_type: $resp_ty }
    }};

    (
        app: $app:expr,
        method: $method:expr,
        path: $path:expr,
        headers: $headers:expr,
        body: { $($body:tt)* }
    ) => {{
        use bdk::prelude::by_axum::axum;
        let body = axum::body::Body::from(serde_json::to_vec(&serde_json::json!({ $($body)* })).unwrap());
        $crate::call! { app: $app, path: $path, method: $method, body: body, headers: $headers }
    }};

    (
        app: $app:expr,
        method: $method:expr,
        path: $path:expr,
        body: { $($body:tt)* }
    ) => {{
        $crate::send! { app: $app, method: $method, path: $path, headers: axum::http::HeaderMap::new(), body: { $($body)* } }
    }};

    (
        app: $app:expr,
        method: $method:expr,
        path: $path:expr,
        headers: $headers:expr,
    ) => {{
        use bdk::prelude::by_axum::axum;
        let body = axum::body::Body::empty();
        $crate::call! { app: $app, path: $path, method: $method, body: body, headers: $headers }
    }};

    (
        app: $app:expr,
        method: $method:expr,
        path: $path:expr,
    ) => {{
        $crate::send! { app: $app, method: $method, path: $path, headers: axum::http::HeaderMap::new() }
    }};
}

/// Sends an HTTP POST request to the given path with optional headers.
/// Notice: End of the macro does not have a comma.
/// Usage: post! { app: ..., path: ..., headers: ..., body: { ... }, response_type: MyType }
#[macro_export]
macro_rules! post {
    (
        app: $app:expr,
        path: $path:expr,
        headers: $headers:expr,
        body: { $($body:tt)* },
        response_type: $resp_ty:ty $(,)?
    ) => {{
        $crate::send! { app: $app, method: "POST", path: $path, headers: $headers, body: { $($body)* }, response_type: $resp_ty }
    }};

    (
        app: $app:expr,
        path: $path:expr,
        body: { $($body:tt)* },
        response_type: $resp_ty:ty $(,)?
    ) => {{
        $crate::post! { app: $app, path: $path, headers: axum::http::HeaderMap::new(), body: { $($body)* }, response_type: $resp_ty }
    }};

    (
        app: $app:expr,
        path: $path:expr,
        headers: $headers:expr,
        body: { $($body:tt)* }
    ) => {{
        $crate::send! { app: $app, method: "POST", path: $path, headers: $headers, body: { $($body)* } }
    }};

    (
        app: $app:expr,
        path: $path:expr,
        body: { $($body:tt)* }
    ) => {{
        $crate::post! { app: $app, path: $path, headers: axum::http::HeaderMap::new(), body: { $($body)* } }
    }};

    (
        app: $app:expr,
        path: $path:expr,
        headers: $headers:expr
    ) => {{
        $crate::send! { app: $app, method: "POST", path: $path, headers: $headers }
    }};

    (
        app: $app:expr,
        path: $path:expr
    ) => {{
        $crate::post! { app: $app, path: $path, headers: axum::http::HeaderMap::new() }
    }};
}

/// Sends an HTTP GET request to the given path with optional headers.
/// Notice: End of the macro does not have a comma.
/// Usage: get! { app: ..., path: ..., headers: ... }
#[macro_export]
macro_rules! get {
    (
        app: $app:expr,
        path: $path:expr,
        headers: $headers:expr
    ) => {{
        $crate::send! { app: $app, method: "GET", path: $path, headers: $headers, }
    }};

    (
        app: $app:expr,
        path: $path:expr
    ) => {{
        $crate::get! { app: $app, path: $path, headers: axum::http::HeaderMap::new() }
    }};
}
