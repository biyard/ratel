use super::*;

pub fn launch(app: fn() -> Element) {
    dioxus::launch(app);
}
