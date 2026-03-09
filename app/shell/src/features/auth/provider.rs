use crate::features::auth::*;

#[component]
pub fn Provider() -> Element {
    #[cfg(feature = "web")]
    {
        use crate::features::auth::interop::initialize;

        let fb_conf = crate::common::FirebaseConfig::default().into();
        initialize(&fb_conf);

        use crate::features::auth::interop::wallet_connect_initialize;
        let wc_conf = crate::common::WalletConnectConfig::default();
        wallet_connect_initialize(&wc_conf);
    }

    rsx! {}
}
