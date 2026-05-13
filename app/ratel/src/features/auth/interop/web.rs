use super::*;

#[cfg(feature = "web")]
define_invoke_js!(
    init_firebase,
    "init_firebase",
    crate::common::FirebaseConfig
);
define_invoke_js!(sign_in, "signIn", res: super::UserInfo);
