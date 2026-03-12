use crate::*;

#[component]
pub fn RootLayout() -> Element {
    rsx! {
        Outlet::<Route> {}
        PopupZone {}
        ToastProvider {}
    }
}
