use std::error::Error;

use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthClient<'a> {
    pub client_id: &'a str,
    pub client_secret: Option<&'a str>,
    pub redirect_uri: &'a str,
    pub scopes: Vec<String>,
    pub token_url: &'a str,
    pub auth_url: &'a str,
    pub revoke_url: Option<&'a str>,
    pub userinfo_url: Option<&'a str>,
    pub openid_url: Option<&'a str>,
}

impl<'a> OAuthClient<'a> {
    pub fn new(
        client_id: &'a str,
        redirect_uri: &'a str,
        auth_url: &'a str,
        token_url: &'a str,
    ) -> Self {
        OAuthClient {
            client_id,
            client_secret: None,
            redirect_uri,
            scopes: vec![],
            token_url,
            auth_url,
            revoke_url: None,
            userinfo_url: None,
            openid_url: None,
        }
    }

    pub fn set_client_secret(&mut self, client_secret: &'a str) -> Self {
        self.client_secret = Some(client_secret);
        self.clone()
    }

    pub fn set_openid_url(&mut self, openid_url: &'a str) -> Self {
        self.openid_url = Some(openid_url);
        self.clone()
    }

    pub fn add_scope(&mut self, scope: &'a str) -> Self {
        self.scopes.push(scope.to_string());
        self.clone()
    }

    pub async fn get_auth_code(&self) -> Result<String, Box<dyn Error>> {
        let mut auth_url = Url::parse(self.auth_url)?;
        auth_url
            .query_pairs_mut()
            .append_pair("response_type", "code")
            .append_pair("client_id", self.client_id)
            .append_pair("redirect_uri", self.redirect_uri);

        if self.scopes.len() > 0 {
            auth_url
                .query_pairs_mut()
                .append_pair("scope", self.scopes.join(" ").as_str());
        }

        let auth_url = auth_url.to_string();
        tracing::debug!("auth_url: {:?}", auth_url);

        #[cfg(feature = "web")]
        {
            use wasm_bindgen::JsCast;

            let window = match web_sys::window() {
                Some(window) => window,
                None => {
                    return Err("Window not found".into());
                }
            };
            let w = match window.open_with_url_and_target_and_features(
                &auth_url.to_string(),
                "",
                "popup",
            ) {
                Ok(Some(w)) => w,
                Ok(None) => {
                    return Err("Window not found".into());
                }
                Err(e) => {
                    return Err(format!("Window open failed: {:?}", e).into());
                }
            };
            tracing::debug!("Window opened: {:?}", w);
            let location = match w.location().href() {
                Ok(location) => location,
                Err(e) => {
                    return Err(format!("Location failed {:?}", e).into());
                }
            };
            tracing::debug!("Location: {:?}", location);
            let code = std::sync::Arc::new(std::sync::RwLock::new(String::new()));

            let promise = web_sys::js_sys::Promise::new(&mut |resolve, _reject| {
                let code_arc = std::sync::Arc::clone(&code);

                let on_message_callback = wasm_bindgen::prelude::Closure::wrap(Box::new(
                    move |event: web_sys::MessageEvent| {
                        if let Some(data) = event.data().as_string() {
                            if data.as_str().starts_with("code=") {
                                let code = data.as_str().replace("code=", "");
                                *code_arc.write().unwrap() = code.clone();
                                tracing::debug!("Code received: {:?}", code);
                                resolve
                                    .call1(&wasm_bindgen::JsValue::NULL, &event.data())
                                    .unwrap();
                            }
                        }
                    },
                )
                    as Box<dyn FnMut(web_sys::MessageEvent)>);

                window.set_onmessage(Some(on_message_callback.as_ref().unchecked_ref()));
                on_message_callback.forget();
            });

            let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
            tracing::debug!("oauth login finished");

            let code = code.read().unwrap();
            tracing::debug!("Code(received): {:?}", code);

            return Ok(code.clone());
        }

        #[allow(unreachable_code)]
        Ok(auth_url)
    }

    pub async fn get_token(&self, code: &str) -> Result<TokenResponse, Box<dyn Error>> {
        let mut params = std::collections::HashMap::new();
        params.insert("grant_type", "authorization_code");
        params.insert("client_id", self.client_id);
        params.insert("redirect_uri", self.redirect_uri);
        params.insert("code", code);

        let client = reqwest::Client::new();
        let res = client.post(self.token_url).form(&params).send().await?;

        Ok(res.json().await?)
    }

    pub async fn get_openid(&self, id_token: &str) -> Result<OpenIdResponse, Box<dyn Error>> {
        if self.openid_url.is_none() {
            return Err("openid_url is not set".into());
        }

        let mut params = std::collections::HashMap::new();
        params.insert("id_token", id_token);

        let client = reqwest::Client::new();
        let res = client
            .post(self.openid_url.unwrap())
            .form(&params)
            .send()
            .await?;

        Ok(res.json().await?)
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct TokenResponse {
    pub access_token: String,
    pub id_token: String,
    pub token_type: String,
    pub expires_in: u64,
}

#[derive(serde::Deserialize, Debug)]
pub struct OpenIdResponse {
    pub iss: String,
    pub sub: String,
    pub nickname: Option<String>,
    pub picture: Option<String>,
    pub email: Option<String>,
}
