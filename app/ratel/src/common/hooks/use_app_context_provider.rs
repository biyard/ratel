use dioxus::core::{provide_root_context, Runtime};

use crate::*;

pub static mut APP_LAYOUT_SCOPE_ID: ScopeId = ScopeId::APP;

pub fn set_app_layout_scope_id(id: ScopeId) {
    unsafe {
        APP_LAYOUT_SCOPE_ID = id;
    }
}

pub fn use_app_context_provider<T: 'static + Clone>(f: impl FnOnce() -> T) -> T {
    use_hook(|| {
        let rt = Runtime::current();
        unsafe { rt.in_scope(APP_LAYOUT_SCOPE_ID, f) }
    })
}
