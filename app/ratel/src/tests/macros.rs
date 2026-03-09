#[macro_export]
macro_rules! test_call {
    (
        app: $app:expr,
        path: $path:expr,
        method: $method:expr,
        body:  $body:expr,
        headers: $headers:expr,
        response_type: $resp_ty:ty
    ) => {{
        use axum::body::HttpBody;
        use axum::http::header::{self, HeaderValue};

        let path = $path.replace("#", "%23");
        let mut req_builder = axum::http::Request::builder()
            .uri(format!("http://localhost:8080{}", path))
            .method($method);

        if let Some(headers_mut) = req_builder.headers_mut() {
            headers_mut.extend($headers);
            let size = $body.size_hint().exact().unwrap_or_default();
            tracing::debug!("Request Body Size: {}", size);
            if size > 0 {
                headers_mut
                    .entry(header::CONTENT_TYPE)
                    .or_insert(HeaderValue::from_static("application/json"));
            }
        }

        let req = req_builder.body($body).unwrap();

        let res: axum::http::Response<axum::body::Body> =
            tower::ServiceExt::oneshot($app.clone(), req).await.unwrap();

        let (parts, body) = res.into_parts();
        let body_bytes = axum::body::to_bytes(body, 10 * 1024 * 1024)
            .await
            .unwrap()
            .to_vec();

        let body_str = String::from_utf8(body_bytes).unwrap();
        tracing::debug!("Response Body: {}", body_str);
        let body = serde_json::from_str::<$resp_ty>(&body_str);
        if let Err(e) = body {
            tracing::error!("Failed to parse response body: {}\nBody: {}", e, body_str);
            (parts.status, parts.headers, <$resp_ty>::default())
        } else {
            (parts.status, parts.headers, body.unwrap())
        }
    }};
}

#[macro_export]
macro_rules! test_send {
    (
        app: $app:expr,
        method: $method:expr,
        path: $path:expr,
        headers: $headers:expr,
        body: { $($body:tt)* },
        response_type: $resp_ty:ty
    ) => {{
        let body = axum::body::Body::from(serde_json::to_vec(&serde_json::json!({ $($body)* })).unwrap());
        $crate::test_call! { app: $app, path: $path, method: $method, body: body, headers: $headers, response_type: $resp_ty }
    }};

    (
        app: $app:expr,
        method: $method:expr,
        path: $path:expr,
        headers: $headers:expr,
        response_type: $resp_ty:ty
    ) => {{
        let body = axum::body::Body::empty();
        $crate::test_call! { app: $app, path: $path, method: $method, body: body, headers: $headers, response_type: $resp_ty }
    }};
}

#[macro_export]
macro_rules! test_http {
    // Two args: app + path
    (@METHOD $method:literal;
     app: $app:expr,
     path: $path:expr $(,)?
    ) => {{
        $crate::test_send! {
            app: $app,
            method: $method,
            path: $path,
            headers: axum::http::HeaderMap::new(),
            response_type: serde_json::Value
        }
    }};

    // Three args: app + path + headers
    (@METHOD $method:literal;
     app: $app:expr,
     path: $path:expr,
     headers: $headers:expr $(,)?
    ) => {{
        $crate::test_send! {
            app: $app,
            method: $method,
            path: $path,
            headers: $headers,
            response_type: serde_json::Value
        }
    }};

    // Three args: app + path + body
    (@METHOD $method:literal;
     app: $app:expr,
     path: $path:expr,
     body: { $($body:tt)* }
    ) => {{
        $crate::test_send! {
            app: $app,
            method: $method,
            path: $path,
            headers: axum::http::HeaderMap::new(),
            body: { $($body)* },
            response_type: serde_json::Value
        }
    }};

    // Three args: app + path + response_type
    (@METHOD $method:literal;
     app: $app:expr,
     path: $path:expr,
     response_type: $resp_ty:ty $(,)?
    ) => {{
        $crate::test_send! {
            app: $app,
            method: $method,
            path: $path,
            headers: axum::http::HeaderMap::new(),
            response_type: $resp_ty
        }
    }};

    // Four args: app + path + body + response_type
    (@METHOD $method:literal;
     app: $app:expr,
     path: $path:expr,
     body: { $($body:tt)* },
     response_type: $resp_ty:ty $(,)?
    ) => {{
        $crate::test_send! {
            app: $app,
            method: $method,
            path: $path,
            headers: axum::http::HeaderMap::new(),
            body: { $($body)* },
            response_type: $resp_ty
        }
    }};

    // Four args: app + path + headers + body
    (@METHOD $method:literal;
     app: $app:expr,
     path: $path:expr,
     headers: $headers:expr,
     body: { $($body:tt)* }
    ) => {{
        $crate::test_send! {
            app: $app,
            method: $method,
            path: $path,
            headers: $headers,
            body: { $($body)* },
            response_type: serde_json::Value
        }
    }};

    // Four args: app + path + headers + response_type
    (@METHOD $method:literal;
     app: $app:expr,
     path: $path:expr,
     headers: $headers:expr,
     response_type: $resp_ty:ty $(,)?
    ) => {{
        $crate::test_send! {
            app: $app,
            method: $method,
            path: $path,
            headers: $headers,
            response_type: $resp_ty
        }
    }};

    // Five args: app + path + headers + body + response_type
    (@METHOD $method:literal;
     app: $app:expr,
     path: $path:expr,
     headers: $headers:expr,
     body: { $($body:tt)* },
     response_type: $resp_ty:ty $(,)?
    ) => {{
        $crate::test_send! {
            app: $app,
            method: $method,
            path: $path,
            headers: $headers,
            body: { $($body)* },
            response_type: $resp_ty
        }
    }};
}

#[macro_export]
macro_rules! test_get {
    ( $($t:tt)* ) => { $crate::test_http!(@METHOD "GET"; $($t)*) };
}

#[macro_export]
macro_rules! test_post {
    ( $($t:tt)* ) => { $crate::test_http!(@METHOD "POST"; $($t)*) };
}

#[macro_export]
macro_rules! test_put {
    ( $($t:tt)* ) => { $crate::test_http!(@METHOD "PUT"; $($t)*) };
}

#[macro_export]
macro_rules! test_patch {
    ( $($t:tt)* ) => { $crate::test_http!(@METHOD "PATCH"; $($t)*) };
}

#[macro_export]
macro_rules! test_delete {
    ( $($t:tt)* ) => { $crate::test_http!(@METHOD "DELETE"; $($t)*) };
}
