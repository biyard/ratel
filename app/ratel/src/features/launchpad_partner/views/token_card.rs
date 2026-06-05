//! "My token holdings" card (arena style) — launchpad-backed on-chain
//! ERC20 balance of the community token for the signed-in user.

use crate::common::*;
use crate::features::launchpad_partner::token_balance::launchpad_token_balance_handler;

#[component]
pub fn LaunchpadTokenCard() -> Element {
    let res = use_loader(move || async move { launchpad_token_balance_handler().await })?;
    let tb = res();

    rsx! {
        div {
            class: "flex justify-between items-center",
            // Same glass tokens the arena `.hero` uses, so the tone matches.
            style: "background: var(--bg-glass); backdrop-filter: blur(16px); -webkit-backdrop-filter: blur(16px); border:1px solid var(--border-subtle); border-radius:20px; padding:24px 28px;",
            span { class: "text-sm font-bold tracking-wide text-foreground-muted", "내 토큰" }
            if tb.has_wallet {
                span {
                    class: "text-2xl font-extrabold",
                    style: "font-family:Orbitron,monospace; color:#6eedd8;",
                    "{tb.balance} "
                    span { class: "text-sm", "{tb.symbol}" }
                }
            } else {
                span { class: "text-sm font-medium text-foreground-muted", "지갑 연결 필요" }
            }
        }
    }
}
