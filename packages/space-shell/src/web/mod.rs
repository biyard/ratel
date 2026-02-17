use common::utils::interop::initialize;

use crate::*;

pub fn launch(app: fn() -> Element) {
    let conf = crate::config::get();
    let fb_conf = conf.common.firebase.into();
    initialize(&fb_conf);

    dioxus::launch(app);
}
