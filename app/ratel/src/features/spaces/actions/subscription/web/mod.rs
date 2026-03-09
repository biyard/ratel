use crate::features::spaces::actions::subscription::*;

pub fn launch(app: fn() -> Element) {
    dioxus::launch(app);
}
