use crate::features::spaces::*;

pub fn launch(app: fn() -> Element) {
    dioxus::launch(app);
}
