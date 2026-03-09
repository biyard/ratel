use crate::features::users::*;

pub fn launch(app: fn() -> Element) {
    dioxus::launch(app);
}
