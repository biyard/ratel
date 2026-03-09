use crate::features::spaces::actions::quiz::*;

pub fn launch(app: fn() -> Element) {
    dioxus::launch(app);
}
