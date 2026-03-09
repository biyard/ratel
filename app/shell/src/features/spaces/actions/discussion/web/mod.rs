use crate::features::spaces::actions::discussion::*;

pub fn launch(app: fn() -> Element) {
    dioxus::launch(app);
}
