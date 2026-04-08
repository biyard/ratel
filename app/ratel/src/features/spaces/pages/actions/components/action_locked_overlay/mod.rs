use crate::common::*;

translate! {
    ActionLockedOverlayTranslate;
    locked_title: {
        en: "This action has started",
        ko: "액션이 시작되었습니다",
    },
    locked_description: {
        en: "Settings can no longer be modified once the action is live. You can still review the configuration in read-only mode.",
        ko: "액션이 시작된 이후에는 설정을 변경할 수 없습니다. 현재 설정은 읽기 전용으로 확인만 가능합니다.",
    },
}

/// Wraps the creator configuration body. When `locked` is `true`:
/// - A banner is rendered at the top explaining that the action has
///   started and settings are read-only.
/// - All interactive controls inside `children` are disabled via
///   `pointer-events-none` on a dimmed (`opacity-60`) wrapper so the
///   creator can still read the current configuration.
///
/// Tabs/other navigation outside the children continue to work
/// normally because they live above this wrapper.
#[component]
pub fn ActionLockedOverlay(locked: bool, children: Element) -> Element {
    let tr: ActionLockedOverlayTranslate = use_translate();

    if !locked {
        return rsx! {
            {children}
        };
    }

    rsx! {
        div { class: "flex flex-col flex-1 gap-4 w-full min-h-0",
            div {
                class: "flex flex-col gap-1 p-4 w-full rounded-lg border border-primary/40 bg-primary/5",
                "data-testid": "action-locked-banner",
                p { class: "text-sm font-semibold text-text-primary", "{tr.locked_title}" }
                p { class: "text-xs text-foreground-muted", "{tr.locked_description}" }
            }
            div {
                class: "flex flex-col flex-1 w-full min-h-0 pointer-events-none opacity-60 select-none",
                aria_disabled: "true",
                "data-testid": "action-locked-content",
                {children}
            }
        }
    }
}
