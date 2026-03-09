use crate::features::posts::*;

pub fn launch(app: fn() -> Element) {
    dioxus::launch(app);
}
