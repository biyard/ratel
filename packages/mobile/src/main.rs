mod app;

use crate::tracing::Level;

use bdk::prelude::*;

#[cfg(not(feature = "server"))]
use app::App;

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    #[cfg(not(feature = "server"))]
    dioxus_aws::launch(App);
}
