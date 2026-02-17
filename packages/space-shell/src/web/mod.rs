use crate::*;

pub fn launch(app: fn() -> Element) {
    dioxus::launch(app);
}
