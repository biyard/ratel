# Dioxus OAuth2

## Add dependancy
- Must add features per server and web.
- This library focuses on web implementation.

``` toml
[dependencies]
dioxus-oauth = { version = "0.1.0", path = "../dioxus-oauth" }

[features]
server = ["dioxus-oauth/server"]
web = ["dioxus-oauth/web"]
```

## Add a component to redirect URI
- By using dioxus-router, add `OAuthPopup` to your redirect URI.

``` rust
use dioxus_oauth::component::OAuthPopup;

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/oauth/code")]
    OAuthPopup { },

}

```

## Add handling an action
- Setting up authorize, token URLs.
- Add button click.

``` rust
#[component]
pub fn OAuthLoginPage() -> Element {
    rsx! {
       div {
           onclick: move |_| {
              #[cfg(feature = "web")]
              {
                  spawn(async move {
                      let client = dioxus_oauth::prelude::OAuthClient::new(
                          env!("CLIENT_ID"),
                          env!("REDIRECT_URI"),
                          "https://kauth.kakao.com/oauth/authorize",
                          "https://kauth.kakao.com/oauth/token",
                      )
                      .set_openid_url("https://kauth.kakao.com/oauth/tokeninfo");

                      let code: String = match client.get_auth_code().await {
                          Ok(code) => code,
                          Err(e) => {
                              tracing::error!("Auth code failed: {:?}", e);
                              return;
                          }
                      };

                      let token_response: dioxus_oauth::prelude::TokenResponse =
                          match client.get_token(code.as_str()).await {
                              Ok(token_response) => token_response,
                              Err(e) => {
                                  tracing::error!("Token response failed: {:?}", e);
                                  return;
                              }
                          };
                      tracing::debug!("Token response: {:?}", token_response);

                      let oid_response: dioxus_oauth::prelude::OpenIdResponse =
                          match client.get_openid(&token_response.id_token).await {
                              Ok(oid_response) => oid_response,
                              Err(e) => {
                                  tracing::error!("Token response failed: {:?}", e);
                                  return;
                              }
                          };

                      tracing::debug!("OID response: {:?}", oid_response);
                  });
              }
           }
       }
    }
}
```
