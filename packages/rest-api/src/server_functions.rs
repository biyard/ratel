use reqwest::RequestBuilder;

use crate::{add_authorization, API_SERVICE, HEADERS, MESSAGE, SIGNER};

pub async fn send(req: RequestBuilder) -> reqwest::Result<reqwest::Response> {
    // let req = run_hooks(req);
    let req = sign_request(req);
    let req = load_headers(req);

    let api_service = unsafe { API_SERVICE.as_mut() };
    let res = match api_service {
        Some(api_service) => api_service.handle(req).await,
        None => req.send().await,
    };

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
        req.header(
            reqwest::header::AUTHORIZATION,
            format!("UserSig {timestamp}:{signature}"),
        )
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

pub fn extract_for_next_request(res: &reqwest::Response) {
    let headers = res.headers();
    if let Some(authz) = headers.get(reqwest::header::AUTHORIZATION) {
        let authz = authz.to_str().unwrap();
        add_authorization(authz);
    } else if let Some(authz) = headers.get("x-amzn-remapped-authorization") {
        let authz = authz.to_str().unwrap();
        add_authorization(authz);
    }
}

pub async fn get<T, E>(url: &str) -> Result<T, E>
where
    T: serde::de::DeserializeOwned,
    E: serde::de::DeserializeOwned + From<reqwest::Error>,
{
    let client = reqwest::Client::builder().build()?;
    #[allow(unused_mut)]
    let mut req = client.get(url);

    // #[cfg(all(feature = "server", not(feature = "test")))]
    // {
    //     use dioxus_fullstack::prelude::server_context;
    //     use reqwest::header::{HeaderMap, AUTHORIZATION, COOKIE};

    //     let ctx = server_context();

    //     let headers: HeaderMap = ctx.extract().await.unwrap();
    //     let auth_token_value = headers
    //         .get(COOKIE)
    //         .and_then(|cookie_header_value| cookie_header_value.to_str().ok())
    //         .and_then(|cookie_str| {
    //             cookie_str.split(';').find_map(|cookie_pair| {
    //                 let mut parts = cookie_pair.trim().splitn(2, '=');
    //                 let key = parts.next()?;
    //                 let value = parts.next()?;
    //                 if key == "auth_token" {
    //                     Some(value.to_string())
    //                 } else {
    //                     None
    //                 }
    //             })
    //         });
    //     if let Some(auth_token_value) = auth_token_value {
    //         tracing::debug!("auth_token_value: {}", auth_token_value);
    //         req = req.header(AUTHORIZATION, auth_token_value)
    //     };
    // }

    let res = send(req).await?;

    if res.status().is_success() {
        return Ok(res.json().await?);
    } else {
        return Err(res.json().await?);
    }
}

pub async fn post<R, T, E>(url: &str, body: R) -> Result<T, E>
where
    R: serde::Serialize,
    T: serde::de::DeserializeOwned,
    E: serde::de::DeserializeOwned + From<reqwest::Error>,
{
    let client = reqwest::Client::builder().build()?;

    let req = client.post(url).json(&body);
    let res = send(req).await?;

    if res.status().is_success() {
        Ok(res.json().await?)
    } else {
        Err(res.json().await?)
    }
}

/// Performs an HTTP GET request.
///
/// # Arguments
///
/// * `url` - The URL to send the request to
/// * `query_params` - Query parameters for the URL. Pass `&None::<()>` to send request without query parameters
///
///
pub async fn get_with_query<T, E, P>(url: &str, query_params: &P) -> Result<T, E>
where
    T: serde::de::DeserializeOwned,
    E: serde::de::DeserializeOwned + From<reqwest::Error>,
    P: serde::Serialize + ?Sized,
{
    let client = reqwest::Client::builder().build()?;

    let req = client.get(url).query(query_params);
    let res = send(req).await?;

    if res.status().is_success() {
        Ok(res.json().await?)
    } else {
        Err(res.json().await?)
    }
}
