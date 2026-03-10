use crate::features::spaces::pages::actions::actions::discussion::*;

pub fn launch(app: fn() -> Element) {
    dioxus::launch(app);
}
