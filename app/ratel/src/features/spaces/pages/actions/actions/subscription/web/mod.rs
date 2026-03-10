use crate::features::spaces::pages::actions::actions::subscription::*;

pub fn launch(app: fn() -> Element) {
    dioxus::launch(app);
}
