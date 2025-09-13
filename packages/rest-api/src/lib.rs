#![allow(static_mut_refs)]
use std::{collections::HashMap, error::Error, future::Future, pin::Pin, sync::RwLock};

use reqwest::RequestBuilder;

pub mod signature;

mod server_functions;
pub use server_functions::*;

pub use signature::Signature;

pub trait Signer {
    fn sign(&self, msg: &str) -> Result<Signature, Box<dyn Error>>;
    fn signer(&self) -> String;
}

pub trait RequestHooker {
    fn before_request(&self, req: RequestBuilder) -> RequestBuilder;
}

pub trait ApiService {
    fn handle(
        &mut self,
        req: RequestBuilder,
    ) -> Pin<Box<dyn Future<Output = Result<reqwest::Response, reqwest::Error>> + Send>>;
}

static mut SIGNER: Option<RwLock<Box<dyn Signer>>> = None;
static mut MESSAGE: Option<String> = None;
// FIXME: It causes dropping Signal of dioxus
// static mut HOOKS: RwLock<Vec<Box<dyn RequestHooker>>> = RwLock::new(Vec::new());
static mut HEADERS: RwLock<Option<HashMap<String, String>>> = RwLock::new(None);
static mut API_SERVICE: Option<Box<dyn ApiService>> = None;

pub fn set_api_service(service: Box<dyn ApiService>) {
    unsafe {
        API_SERVICE = Some(service);
    }
}
// pub fn add_hook<T: RequestHooker + 'static>(hook: T) {
//     unsafe {
//         HOOKS.write().unwrap().push(Box::new(hook));
//     }
// }

// pub fn run_hooks(req: RequestBuilder) -> RequestBuilder {
//     unsafe {
//         HOOKS
//             .read()
//             .unwrap()
//             .iter()
//             .fold(req, |req, hook| hook.before_request(req))
//     }
// }

pub fn get_authz_token() -> Option<String> {
    unsafe {
        let headers = HEADERS.read().unwrap();
        match headers.as_ref() {
            Some(headers) => headers
                .get("Authorization")
                .cloned()
                .unwrap_or_default()
                .split(" ")
                .last()
                .map(|s| s.to_string()),
            _ => None,
        }
    }
}

pub fn get_header(key: &str) -> Option<String> {
    unsafe {
        let headers = HEADERS.read().unwrap();
        match headers.as_ref() {
            Some(headers) => headers.get(key).cloned(),
            None => None,
        }
    }
}

pub fn add_header(key: String, value: String) {
    unsafe {
        let mut headers = HEADERS.write().unwrap();
        match headers.as_mut() {
            Some(headers) => {
                headers.insert(key, value);
            }
            None => {
                let mut new_headers = HashMap::new();
                new_headers.insert(key, value);
                *headers = Some(new_headers);
            }
        }
    }
}

pub fn remove_header(key: &str) {
    unsafe {
        let mut headers = HEADERS.write().unwrap();
        match headers.as_mut() {
            Some(headers) => {
                headers.remove(key);
            }
            None => {}
        }
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

pub fn set_message(msg: String) {
    unsafe {
        MESSAGE = Some(msg);
    }
}

pub fn add_authorization(token: &str) {
    unsafe {
        let mut headers = HEADERS.write().unwrap();
        match headers.as_mut() {
            Some(headers) => {
                headers.insert("Authorization".to_string(), token.to_string());
            }
            None => {
                let mut new_headers = HashMap::new();
                new_headers.insert("Authorization".to_string(), token.to_string());
                *headers = Some(new_headers);
            }
        }
    }
}
