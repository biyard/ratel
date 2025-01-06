use std::sync::RwLock;

use dto::*;
use reqwest::RequestBuilder;
use serde::Serialize;

static mut SIGNER: Option<RwLock<Box<dyn Signer>>> = None;

pub fn set_signer(signer: Box<dyn Signer>) {
    unsafe {
        SIGNER = Some(RwLock::new(signer));
    }
}

pub fn remove_signer() {
    unsafe {
        SIGNER = None;
    }
}

pub fn sign_request(req: RequestBuilder) -> RequestBuilder {
    tracing::debug!("Signing request");
    #[allow(static_mut_refs)]
    if let Some(signer) = unsafe { &SIGNER } {
        let signer = signer.read().unwrap();
        let address = signer.signer();
        if address.is_empty() {
            return req;
        }

        let conf = ::get();

        let timestamp = chrono::Utc::now().timestamp();
        let msg = format!("{}-{}", conf.domain, timestamp);
        let signature = signer.sign(&msg);
        if signature.is_err() {
            return req;
        }

        let signature = signature.unwrap();
        tracing::debug!("Signature: {:?}", signature);
        req.header("Authorization", format!("UserSig {timestamp}:{signature}"))
    } else {
        req
    }
}

pub async fn get<T>(url: &str) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let client = reqwest::Client::builder().build()?;

    let req = client.get(url);
    #[cfg(feature = "web-only")]
    let req = sign_request(req);
    let res = req.send().await?;

    if res.status().is_success() {
        Ok(res.json().await?)
    } else {
        Err(res.json().await?)
    }
}

pub async fn post<R, T>(url: &str, body: R) -> Result<T>
where
    R: Serialize,
    T: serde::de::DeserializeOwned,
{
    let client = reqwest::Client::builder().build()?;

    let req = client.post(url).json(&body);

    #[cfg(feature = "web-only")]
    let req = sign_request(req);

    let res = req.send().await?;

    if res.status().is_success() {
        Ok(res.json().await?)
    } else {
        Err(res.json().await?)
    }
}
