use crate::features::spaces::actions::poll::*;

pub fn launch(app: fn() -> Element) {
    dioxus::launch(app);
}
