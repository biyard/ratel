use crate::*;

#[derive(Clone, PartialEq)]
pub struct RedirectTo {
    pub target: Route,
    pub label: String,
}

#[component]
pub fn WrappedPage(children: Element, redirection: ReadSignal<Option<RedirectTo>>) -> Element {
    rsx! {
        ErrorBoundary {
            handle_error: move |error: ErrorContext| {
                error!("Error in component tree: {:?}", error);
                rsx! {
                    ErrorPage { ctx: error, redirection }
                }
            },
            SuspenseBoundary { {children} }
        }
    }
}

#[component]
pub fn ErrorPage(ctx: ErrorContext, redirection: ReadSignal<Option<RedirectTo>>) -> Element {
    let tr: ErrorPageTranslate = use_translate();
    let lang = use_language();
    let nav = use_navigator();
    let mut user_ctx = crate::features::auth::hooks::use_user_context();
    let label = redirection()
        .map(|r| r.label)
        .unwrap_or(tr.go_home.to_string());

    let (is_auth_error, message) = match ctx.error() {
        Some(err) => match err.downcast_ref::<Error>() {
            Some(typed) => {
                let is_auth = matches!(
                    typed,
                    Error::NoSessionFound
                        | Error::UnauthorizedAccess
                        | Error::UserNotFoundInContext
                );
                (is_auth, typed.translate(&lang()).to_string())
            }
            None => (false, format!("{err:?}")),
        },
        None => (false, tr.description.to_string()),
    };

    rsx! {
        div { class: "flex flex-col gap-6 justify-center items-center py-16 px-6 w-full min-h-screen bg-background",
            div { class: "flex flex-col gap-3 items-center max-w-md text-center",
                h1 { class: "text-2xl font-bold text-text-primary", "{tr.title}" }
                p { class: "text-sm text-foreground-muted", "{message}" }
            }
            div { class: "flex flex-row gap-3",
                Button {
                    style: ButtonStyle::Primary,
                    shape: ButtonShape::Rounded,
                    onclick: {
                        let ctx = ctx;
                        move |_| {
                            if is_auth_error {
                                user_ctx.set(crate::features::auth::context::UserContext::default());
                                crate::common::services::persistent_state::clear_cached_session();
                            }
                            let target = redirection().map(|r| r.target).unwrap_or(Route::Index {});
                            ctx.clear_errors();
                            nav.push(target);
                        }
                    },
                    "{label}"
                }
            }
        }
    }
}

translate! {
    ErrorPageTranslate;

    title: {
        en: "Something went wrong",
        ko: "문제가 발생했습니다",
    },

    description: {
        en: "We couldn't load this page.",
        ko: "페이지를 불러올 수 없습니다.",
    },

    go_home: {
        en: "Go home",
        ko: "홈으로 이동",
    },
}
