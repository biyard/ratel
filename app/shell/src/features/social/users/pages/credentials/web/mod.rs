use crate::features::social::users::*;

pub fn launch(app: fn() -> Element) {
    dioxus::launch(app);
}
