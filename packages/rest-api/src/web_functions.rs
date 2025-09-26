use gloo_net::http::RequestBuilder;
use gloo_net::http::Response;
const AUTHORIZATION: &str = "Authorization";

use crate::add_authorization;
use crate::{HEADERS, MESSAGE, SIGNER};

pub async fn send(req: RequestBuilder) -> Result<gloo_net::http::Response, gloo_net::Error> {
    // let req = run_hooks(req);
    let req = sign_request(req);
    let req = load_headers(req);

    let res = req.send().await;

    if let Ok(res) = &res {
        extract_for_next_request(res);
    }

    res
}

pub async fn send_with_body(
    req: RequestBuilder,
) -> Result<gloo_net::http::Response, gloo_net::Error> {
    // let req = run_hooks(req);
    let req = sign_request(req);
    let req = load_headers(req);

    let res = req.send().await;

    if let Ok(res) = &res {
        extract_for_next_request(res);
    }

    res
}

pub fn sign_request(req: RequestBuilder) -> RequestBuilder {
    if let (Some(signer), Some(msg)) = unsafe { (&SIGNER, &MESSAGE) } {
        let signer = signer.read().unwrap();
        let address = signer.signer();
        tracing::debug!("Signer address: {}", address);
        if address.is_empty() {
            return req;
        }

        let timestamp = chrono::Utc::now().timestamp();
        let msg = format!("{}-{}", msg, timestamp);
        tracing::debug!("Signing message: {}", msg);
        let signature = signer.sign(&msg);
        if signature.is_err() {
            return req;
        }

        let signature = signature.unwrap();
        req.header(AUTHORIZATION, &format!("UserSig {timestamp}:{signature}"))
    } else {
        tracing::debug!("No signer found");
        req
    }
}

pub fn load_headers(mut req: RequestBuilder) -> RequestBuilder {
    unsafe {
        match HEADERS.read().unwrap().as_ref() {
            Some(ref headers) => {
                for (k, v) in headers.iter() {
                    req = req.header(k, v);
                }

                req
            }
            None => req,
        }
    }
}

pub fn extract_for_next_request(res: &Response) {
    let headers = res.headers();
    if let Some(authz) = headers.get(AUTHORIZATION) {
        add_authorization(&authz);
    } else if let Some(authz) = headers.get("x-amzn-remapped-authorization") {
        add_authorization(&authz);
    }
}

pub async fn get<T, E>(url: &str) -> Result<T, E>
where
    T: serde::de::DeserializeOwned,
    E: serde::de::DeserializeOwned + From<gloo_net::Error>,
{
    let req = gloo_net::http::Request::get(url)
        .header("Content-Type", "application/json")
        .credentials(web_sys::RequestCredentials::Include);
    let res = send(req).await?;
    let status_code = res.status();
    if status_code < 400 {
        Ok(res.json().await?)
    } else {
        Err(res.json().await?)
    }
}

pub async fn post<R, T, E>(url: &str, body: R) -> Result<T, E>
where
    R: serde::Serialize,
    T: serde::de::DeserializeOwned,
    E: serde::de::DeserializeOwned + From<gloo_net::Error>,
{
    tracing::debug!("POST(Web) {}", url);
    let req = gloo_net::http::Request::post(url)
        .header("Content-Type", "application/json")
        .credentials(web_sys::RequestCredentials::Include);
    let req = sign_request(req);
    let req = load_headers(req);
    let req = req.json(&body)?;

    let res = req.send().await?;

    extract_for_next_request(&res);

    let status_code = res.status();
    if status_code < 400 {
        Ok(res.json().await?)
    } else {
        Err(res.json().await?)
    }
}
