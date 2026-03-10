use crate::features::spaces::pages::actions::actions::poll::*;

pub fn launch(app: fn() -> Element) {
    dioxus::launch(app);
}
