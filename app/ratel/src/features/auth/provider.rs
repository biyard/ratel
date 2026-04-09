use crate::features::auth::*;

#[component]
pub fn Provider() -> Element {
    #[cfg(all(feature = "web", not(feature = "server")))]
    {
        use crate::features::auth::interop::init_firebase;

        let fb_conf = crate::common::FirebaseConfig::default().into();
        init_firebase(&fb_conf);

        use crate::features::auth::interop::wallet_connect_initialize;
        let wc_conf = crate::common::WalletConnectConfig::default();
        wallet_connect_initialize(&wc_conf);
    }

    rsx! {}
}
