use hello::hello_handler;

use crate::*;

mod hello;

pub fn route() -> Result<Router> {
    Ok(Router::new().route("/", get(hello_handler)))
}
