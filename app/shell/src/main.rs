#[allow(unused_imports)]
use app_shell::config;
#[allow(unused_imports)]
use app_shell::*;
#[allow(unused_imports)]
use common::*;

#[allow(unused_imports)]
use dioxus::prelude::*;

fn main() {
    let config = config::get();
    common::logger::init(config.common.log_level.into()).expect("logger failed to init");
    debug!("Config: {:#?}", config);

    #[cfg(not(feature = "server"))]
    web::launch(App);

    #[cfg(feature = "server")]
    server::serve(App);
}
