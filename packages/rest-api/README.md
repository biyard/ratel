# REST API

## Usage

### Register hook

``` rust
fn init() {
  let hooker = Hook::new();
  rest_api::add_hook(hooker);
}

struct Hook {}

impl rest_api::RequestHooker for Hook {
    fn before_request(&self, req: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        req.header("Authorization", format!("Bearer token"))
    }
}
```

### Call API
- Below code will call `before_request` function

``` rust
rest_api::get("http://localhost:3000/version");
```
