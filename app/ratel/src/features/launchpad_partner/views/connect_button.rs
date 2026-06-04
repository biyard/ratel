//! Per-user "Convert on Launchpad" entry point. Loads the encrypted
//! handoff URL from the server (`launchpad_entry_url_handler`) so the
//! token-bearing href is identical on SSR and after hydration.

use crate::features::launchpad_partner::LaunchpadPartnerTranslate;
use crate::features::launchpad_partner::entry::launchpad_entry_url_handler;
use crate::*;

#[component]
pub fn LaunchpadConnectButton() -> Element {
    let tr: LaunchpadPartnerTranslate = use_translate();
    let url_loader = use_loader(move || async move { launchpad_entry_url_handler().await })?;
    let url = url_loader().url;

    rsx! {
        a {
            class: "launchpad-connect-btn",
            href: "{url}",
            "{tr.convert_cta}"
        }
    }
}
