use crate::*;

#[component]
pub fn AuthProvider() -> Element {
    #[cfg(feature = "web")]
    {
        use crate::interop::initialize;

        let fb_conf = FirebaseConfig::default().into();
        initialize(&fb_conf);
    }

    rsx! {
        document::Script { src: asset!("/assets/ratel-auth.js") }
    }
}
