use crate::*;

#[component]
pub fn RootLayout() -> Element {
    let lang = use_language();

    rsx! {
        ErrorBoundary {
            handle_error: move |error: ErrorContext| {
                error!("Error in component tree: {:?}", error);

                if let Some(err) = error.error() {
                    if let Some(e) = err.downcast_ref::<Error>() {
                        return rsx! {
                            div { {e.translate(&lang())} }
                        };
                    }
                }

                error.clear_errors();
                rsx! {
                    div { "Oops, we encountered an error" }
                }
            },
            SuspenseBoundary { Outlet::<Route> {} }
        }
        PopupZone {}
        ToastProvider {}
    }
}
