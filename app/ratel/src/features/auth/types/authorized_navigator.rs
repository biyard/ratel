use crate::{auth::LoginModal, *};

pub trait AuthorizedNavigator {
    fn auth_push(&self, route: Route);
}

impl AuthorizedNavigator for dioxus_router::Navigator {
    fn auth_push(&self, route: Route) {
        let AuthContext { logged_in, .. } = crate::features::auth::hooks::use_auth_context();
        if logged_in() {
            self.push(route);
        } else {
            let mut popup = consume_popup();
            let r = route.clone();
            let nav = *self;
            popup
                .open(rsx! {
                    LoginModal {
                        on_success: move |_| {
                            nav.push(r.clone());
                        },
                    }
                })
                .with_title("Start building your Essence");
        }
    }
}
