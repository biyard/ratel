use crate::*;

#[component]
pub fn Provider() -> Element {
    #[cfg(feature = "web")]
    {
        use crate::interop::initialize;

        let fb_conf = common::FirebaseConfig::default().into();
        initialize(&fb_conf);
    }

    rsx! {
        document::Script { src: asset!("/assets/ratel-auth.js") }
    }
}
