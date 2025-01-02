use base64::{engine::general_purpose, Engine};
use std::{fmt::Display, sync::RwLock};

use dto::Result;
use reqwest::RequestBuilder;
use serde::Serialize;

pub trait Signer {
    fn sign(&self, msg: &str) -> Result<Signature>;
    fn signer(&self) -> String;
}

static mut SIGNER: Option<RwLock<Box<dyn Signer>>> = None;

#[derive(Debug, Clone)]
pub enum SignatureAlgorithm {
    EdDSA,
}

impl Display for SignatureAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "eddsa")
    }
}

#[derive(Debug, Clone)]
pub struct Signature {
    pub signature: Vec<u8>,
    pub algorithm: SignatureAlgorithm,
    pub public_key: String,
}

impl Display for Signature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sig = general_purpose::STANDARD.encode(&self.signature);

        write!(f, "{}:{}:{}", self.algorithm, self.public_key, sig)
    }
}

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
    if let Some(signer) = unsafe { &SIGNER } {
        let signer = signer.read().unwrap();
        let address = signer.signer();
        if address.is_empty() {
            return req;
        }

        let conf = crate::config::get();

        let timestamp = chrono::Utc::now().timestamp();
        let msg = format!("{}-{}", conf.domain, timestamp);
        let signature = signer.sign(&msg);
        if signature.is_err() {
            return req;
        }

        let signature = signature.unwrap();

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

    let req = if body.is_some() {
        client.post(url).json(&body)
    } else {
        client.post(url)
    };
    #[cfg(feature = "web-only")]
    let req = sign_request(req);

    let res = req.send().await?;

    if res.status().is_success() {
        Ok(res.json().await?)
    } else {
        Err(res.json().await?)
    }
}
