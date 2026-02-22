use crate::controllers::get_me::get_me_handler;
use crate::*;

#[component]
pub fn AuthProvider() -> Element {
    #[cfg(feature = "web")]
    {
        use crate::interop::initialize;

        let fb_conf = common::FirebaseConfig::default().into();
        initialize(&fb_conf);
    }

    let mut user_ctx = crate::hooks::use_user_context();

    // Restore session on page load
    use_server_future(move || async move {
        match get_me_handler().await {
            Ok(resp) => {
                if resp.user.is_some() {
                    user_ctx.set(UserContext {
                        user: resp.user,
                        refresh_token: None,
                    });
                }
            }
            Err(e) => {
                tracing::debug!("No active session: {:?}", e);
            }
        }
        Ok::<(), ()>(())
    });

    rsx! {
        document::Script { src: asset!("/assets/ratel-auth.js") }
    }
}
