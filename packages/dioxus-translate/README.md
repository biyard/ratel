# Dioxus Translate


## Usage
### Add dependancy
- `Cargo.toml`

``` toml
[dependencies]
dioxus-translate = { version = "0.1.0" }

[features]
web-only = ["dioxus-translate/web"]
```

### Define translate
- Define `i18n` translates.

``` rust
use dioxus_translate::translate;

translate! {
    HeaderTranslate;

    signup: {
        en: "Sign up",
    },

    login: {
        en: "Login",
    },
}
```

- The above macro will be expanded to the below code

``` rust
#[derive(Debug, Clone, PartialEq)]
pub struct HeaderTranslate {
    pub signup: &'static str,
    pub login: &'static str,
}

impl dioxus_translate::Translator for HeaderTranslate {
    fn en() -> Self {
        Self {
            signup: "Sign up",
            login: "Login",
        }
    }
}
```

- At this, `Translator` define the below `Translator`.

``` rust
pub trait Translator {
    fn en() -> Self;
}
```

### Getting translate

``` rust
#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn Component(lang: Language) -> Element {
    let i18n_header: HeaderTranslate = translate(&lang);

    rsx! { "{i18n_header.signup}" }
}

```

## Define additional language spec
- Currently, only `ko` language is supported with `ko` feature

### Add `ko` feature
- Add `ko` feature in `Cargo.toml`

``` toml
[dependencies]
dioxus-translate = { version = "0.1.0", features = ["ko"] }
```

### Define translate
``` rust
use dioxus_translate::translate;

translate! {
    HeaderTranslate;

    signup: {
        ko: "회원가입",
        en: "Sign up",
    },

    login: {
        ko: "로그인",
        en: "Login",
    },
}
```
