use crate::features::my_follower::*;

pub fn launch(app: fn() -> Element) {
    dioxus::launch(app);
}
