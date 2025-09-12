use axum::{
    body::{to_bytes, Body, Bytes},
    http::{Request, Response},
    Router,
};
use reqwest::RequestBuilder;
use rest_api::ApiService;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tower::{Service, ServiceExt};

#[derive(Clone)]
pub struct RestApiAdapter {
    router: Router,
}

impl RestApiAdapter {
    pub fn new(router: Router) -> Self {
        Self { router }
    }
}

impl Service<RequestBuilder> for RestApiAdapter {
    type Response = reqwest::Response;
    type Error = reqwest::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req_builder: RequestBuilder) -> Self::Future {
        let router = self.router.clone();

        Box::pin(async move {
            let req = req_builder.build().unwrap();
            let uri = req.url().as_str();

            let mut axum_req = Request::builder().uri(uri).method(req.method().as_str());

            for (key, value) in req.headers().iter() {
                axum_req = axum_req.header(key, value);
            }

            let body = match req.body() {
                Some(body) => {
                    let bytes = body
                        .as_bytes()
                        .ok_or("Failed to extract body as bytes")
                        .unwrap();
                    Body::from(Bytes::copy_from_slice(bytes))
                }
                None => Body::empty(),
            };

            let axum_req = axum_req.body(body).unwrap();
            let res: Response<Body> = router.oneshot(axum_req).await.unwrap();

            let limit = res
                .headers()
                .get("content-length")
                .and_then(|v| v.to_str().ok().and_then(|s| s.parse::<usize>().ok()));
            let headers = res.headers().clone();
            let status = res.status();

            let body: Body = res.into_body();
            let body_bytes: Bytes = to_bytes(body, limit.unwrap_or(1024 * 1024 * 10))
                .await
                .unwrap();
            let body_bytes: Vec<u8> = (*body_bytes).to_vec();

            let mut builder = ::http::Response::builder();
            for (key, value) in headers.iter() {
                builder = builder.header(key, value);
            }

            builder = builder.status(status);

            let res = reqwest::Response::from(builder.body(body_bytes).unwrap());

            Ok(res)
        })
    }
}

impl ApiService for RestApiAdapter {
    fn handle(
        &mut self,
        req: RequestBuilder,
    ) -> Pin<Box<dyn Future<Output = Result<reqwest::Response, reqwest::Error>> + Send>> {
        self.call(req)
    }
}
