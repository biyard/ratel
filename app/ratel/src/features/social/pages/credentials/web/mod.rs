use crate::features::social::*;

pub fn launch(app: fn() -> Element) {
    dioxus::launch(app);
}
