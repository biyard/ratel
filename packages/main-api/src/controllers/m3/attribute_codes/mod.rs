mod create_attiribute_code;
mod delete_attribute_code;
mod list_attribute_codes;

use crate::features::did::AttributeCode;
use crate::*;

pub fn route() -> Result<Router<AppState>> {
    Ok(Router::new()
        .route(
            "/",
            post(create_attiribute_code::create_attiribute_code_handler)
                .get(list_attribute_codes::list_attribute_codes_handler),
        )
        .route(
            "/:code_pk",
            delete(delete_attribute_code::delete_attribute_code_handler),
        ))
}
