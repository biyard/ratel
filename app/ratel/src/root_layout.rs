use crate::common::components::{Button, ButtonShape, ButtonStyle};
use crate::*;

#[component]
pub fn RootLayout() -> Element {
    rsx! {
        ErrorBoundary {
            handle_error: move |error: ErrorContext| {
                error!("Error in component tree: {:?}", error);
                rsx! {
                    ErrorPage { ctx: error }
                }
            },
            SuspenseBoundary { Outlet::<Route> {} }
        }
        PopupZone {}
        ToastProvider {}
    }
}

#[component]
fn ErrorPage(ctx: ErrorContext) -> Element {
    let tr: ErrorPageTranslate = use_translate();
    let lang = use_language();
    let nav = use_navigator();

    let message = if let Some(err) = ctx.error() {
        if let Some(e) = err.downcast_ref::<Error>() {
            e.translate(&lang()).to_string()
        } else {
            format!("{err:?}")
        }
    } else {
        tr.description.to_string()
    };

    rsx! {
        div { class: "flex flex-col gap-6 justify-center items-center px-6 py-16 w-full min-h-screen bg-background",
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
                            ctx.clear_errors();
                            nav.push(Route::Index {});
                        }
                    },
                    "{tr.go_home}"
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
