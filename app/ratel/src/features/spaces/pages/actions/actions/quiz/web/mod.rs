use crate::features::spaces::pages::actions::actions::quiz::*;

pub fn launch(app: fn() -> Element) {
    dioxus::launch(app);
}
