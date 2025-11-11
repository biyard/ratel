#[allow(missing_docs)]
pub struct LambdaAdapter<'a, R, S> {
    service: S,
    _phantom_data: std::marker::PhantomData<&'a R>,
}

impl<'a, R, S, E> From<S> for LambdaAdapter<'a, R, S>
where
    S: tower::Service<lambda_http::Request, Response = R, Error = E>,
    S::Future: Send + 'a,
    R: lambda_http::IntoResponse,
{
    fn from(service: S) -> Self {
        LambdaAdapter {
            service,
            _phantom_data: std::marker::PhantomData,
        }
    }
}

impl<'a, R, S, E> tower::Service<lambda_http::LambdaEvent<lambda_http::request::LambdaRequest>>
    for LambdaAdapter<'a, R, S>
where
    S: tower::Service<lambda_http::Request, Response = R, Error = E>,
    S::Future: Send + 'a,
    R: lambda_http::IntoResponse,
{
    type Response = lambda_http::aws_lambda_events::apigw::ApiGatewayProxyResponse;

    type Error = E;
    type Future = std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send + 'a>,
    >;

    fn poll_ready(
        &mut self,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(
        &mut self,
        req: lambda_http::LambdaEvent<lambda_http::request::LambdaRequest>,
    ) -> Self::Future {
        // tracing::debug!("Incoming request: {:?}", req);

        // Extract raw path and query string from API Gateway event before decoding
        let (raw_path, raw_query_string, stage) = match &req.payload {
            lambda_http::request::LambdaRequest::ApiGatewayV2(apigw_req) => {
                // V2 provides raw_path and raw_query_string which are percent-encoded
                (
                    apigw_req.raw_path.clone(),
                    apigw_req.raw_query_string.clone(),
                    None,
                )
            }
            lambda_http::request::LambdaRequest::ApiGatewayV1(apigw_req) => {
                // V1 only provides decoded path, we'll use it as-is
                // Note: V1 doesn't provide raw query string, only decoded parameters
                (
                    apigw_req.path.clone(),
                    None,
                    apigw_req.request_context.stage.clone(),
                )
            }
            lambda_http::request::LambdaRequest::Alb(alb_req) => {
                // ALB only provides decoded path
                (alb_req.path.clone(), None, None)
            }
            _ => (None, None, None),
        };

        let mut event: lambda_http::Request = req.payload.into();

        // Construct raw URI from raw path and query without decoding
        if let Some(path) = raw_path {
            // Build the raw URI with percent-encoded path and query
            let raw_uri = if let Some(query) = raw_query_string {
                if query.is_empty() {
                    path
                } else {
                    format!("{}?{}", path, query)
                }
            } else {
                path
            };

            // Remove stage prefix if present (for API Gateway V1)
            let final_uri = if let Some(stage) = stage {
                let stage_prefix = format!("/{}", stage);
                raw_uri.replacen(&stage_prefix, "", 1)
            } else {
                raw_uri
            };

            // Set the raw, percent-encoded URI directly
            *event.uri_mut() = final_uri.parse().unwrap_or_else(|e| {
                tracing::warn!("Failed to parse raw URI '{}': {}", final_uri, e);
                event.uri().clone()
            });
        }

        // tracing::debug!("manipulated event requests: {:?}", event);

        let call = self
            .service
            .call(lambda_http::RequestExt::with_lambda_context(
                event,
                req.context,
            ));

        Box::pin(async move {
            let res = call.await?;
            let res = res.into_response().await;
            let status_code = res.status().as_u16() as i64;
            let headers = res.headers().clone();
            let body = Some(res.clone().into_body());

            let res = lambda_http::aws_lambda_events::apigw::ApiGatewayProxyResponse {
                is_base64_encoded: false,
                status_code,
                headers,
                body,
                multi_value_headers: Default::default(),
            };

            Ok(res)
        })
    }
}
