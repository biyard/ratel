use dioxus::prelude::*;

pub fn launch(app: fn() -> Element) {
    dioxus::launch(app);
}
