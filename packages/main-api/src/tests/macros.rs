#[macro_export]
macro_rules! call {
    (
        app: $app:expr,
        path: $path:expr,
        method: $method:expr,
        body:  $body:expr
    ) => {{
        use bdk::prelude::by_axum::axum;

        let req = axum::http::Request::builder()
            .uri(concat!("http://localhost:3000", $path))
            .method($method)
            .header("content-type", "application/json")
            .body($body)
            .unwrap();

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
        body: { $($body:tt)* },
        response_type: $resp_ty:ty
    ) => {{
        use bdk::prelude::by_axum::axum;

        let body = axum::body::Body::from(
            serde_json::to_vec(&serde_json::json!({ $($body)* })).unwrap()
        );

        let (status, headers, parsed) = $crate::call! { app: $app, path: $path, method: $method, body: body };

        let parsed = serde_json::from_str::<$resp_ty>(&parsed).unwrap();

        (status, headers, parsed)
    }};

    (
        app: $app:expr,
        method: $method:expr,
        path: $path:expr,
        body: { $($body:tt)* }
    ) => {{
        use bdk::prelude::by_axum::axum;

        let body = axum::body::Body::from(
            serde_json::to_vec(&serde_json::json!({ $($body)* })).unwrap()
        );

        $crate::call! { app: $app, path: $path, method: $method, body: body }
    }};

    (
        app: $app:expr,
        method: $method:expr,
        path: $path:expr,
    ) => {{
        use bdk::prelude::by_axum::axum;

        let body = axum::body::Body::empty();

        $crate::call! { app: $app, path: $path, method: $method, body: body }
    }};
}

#[macro_export]
macro_rules! post {
    (
        app: $app:expr,
        path: $path:expr,
        body: { $($body:tt)* },
        response_type: $resp_ty:ty $(,)?
    ) => {{
        $crate::send! { app: $app, method: "POST", path: $path, body: { $($body)* }, response_type: $resp_ty }
    }};

    (
        app: $app:expr,
        path: $path:expr,
        body: { $($body:tt)* },
    ) => {{
        $crate::send! { app: $app, method: "POST", path: $path, body: { $($body)* } }
    }};

    (
        app: $app:expr,
        path: $path:expr,
    ) => {{
        $crate::send! { app: $app, method: "POST", path: $path }
    }};
}

#[macro_export]
macro_rules! post_with_body {
    (
        app: $app:expr,
        path: $path:expr,
        body: { $($body:tt)* },
        response_type: $resp_ty:ty $(,)?
    ) => {{
        $crate::post! { app: $app, path: $path, body: { $($body)* }, response_type: $resp_ty }
    }};
}
