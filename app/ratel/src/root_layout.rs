use crate::*;

#[component]
pub fn RootLayout() -> Element {
    rsx! {
        PopupZone {}
        ToastProvider {}
    }
}
