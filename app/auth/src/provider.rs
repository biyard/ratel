use crate::*;

#[component]
pub fn Provider() -> Element {
    #[cfg(feature = "web")]
    {
        use crate::interop::initialize;

        let fb_conf = common::FirebaseConfig::default().into();
        initialize(&fb_conf);

        use crate::interop::wallet_connect_initialize;
        let wc_conf = common::WalletConnectConfig::default();
        wallet_connect_initialize(&wc_conf);
    }

    rsx! {
        document::Script { src: asset!("/assets/ratel-auth.js") }
    }
}
