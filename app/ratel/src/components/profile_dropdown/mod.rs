use crate::*;

translate! {
    ProfileDropdownTranslate;

    teams: {
        en: "Teams",
        ko: "팀",
    },

    create_team: {
        en: "Create Team",
        ko: "팀 생성",
    },

    logout: {
        en: "Log Out",
        ko: "로그아웃",
    },

    delete_account: {
        en: "Delete Account",
        ko: "회원 탈퇴",
    },

    delete_account_title: {
        en: "Delete your account?",
        ko: "회원 탈퇴하시겠습니까?",
    },

    delete_account_warning: {
        en: "Your account and profile will be permanently deleted and cannot be recovered. You won't be able to restore your account information, and signing in again will create a new, empty account.",
        ko: "계정과 프로필이 영구적으로 삭제되며 복구할 수 없습니다. 회원 정보는 다시 복구되지 않으며, 같은 정보로 다시 가입하면 새로운 빈 계정이 생성됩니다.",
    },

    cancel: {
        en: "Cancel",
        ko: "취소",
    },

    confirm_delete: {
        en: "Delete Account",
        ko: "탈퇴하기",
    },

    delete_failed: {
        en: "Failed to delete your account. Please try again.",
        ko: "회원 탈퇴에 실패했습니다. 다시 시도해 주세요.",
    },
}

#[component]
pub fn ProfileDropdown() -> Element {
    let tr: ProfileDropdownTranslate = use_translate();
    let user_ctx = crate::features::auth::hooks::use_user_context();
    let team_ctx = use_team_context();
    let mut open = use_signal(|| false);
    let mut popup = use_popup();
    let nav = use_navigator();

    let user = user_ctx().user.clone();
    let Some(user) = user else {
        return rsx! {
            div {}
        };
    };

    let profile_url = user.profile_url.clone();
    let display_name = user.display_name.clone();

    let teams = team_ctx.teams.read().clone();

    rsx! {
        div { class: "relative",
            // Trigger button
            button {
                class: "flex flex-col justify-center items-center p-2.5 cursor-pointer group",
                onclick: move |_| {
                    open.set(!open());
                },
                if !profile_url.is_empty() {
                    img {
                        src: "{profile_url}",
                        alt: "User Profile",
                        class: "object-cover w-6 h-6 rounded-full",
                    }
                } else {
                    div { class: "w-6 h-6 rounded-full bg-neutral-500" }
                }
                span { class: "font-medium transition-colors text-menu-text text-[15px] max-w-20 truncate group-hover:text-menu-text/80",
                    "{display_name}"
                }
            }

            // Dropdown
            if open() {
                // Backdrop to close on click outside
                div {
                    class: "fixed inset-0 z-998",
                    onclick: move |_| {
                        open.set(false);
                    },
                }

                div { class: "absolute right-0 top-full p-2.5 rounded-lg border w-[250px] border-divider bg-bg z-999",
                    // Teams label
                    div { class: "px-2 text-xs text-c-secondary py-1", "{tr.teams}" }

                    // Scrollable team list
                    div { class: "overflow-y-auto pr-2 -mr-2 max-h-[300px]",
                        // User entry (index 0)
                        Link {
                            class: "flex gap-2 items-center py-1.5 px-2 w-full rounded-md cursor-pointer hover:bg-hover",
                            to: format!("/"),
                            onclick: move |_| {
                                open.set(false);
                            },
                            if !user.profile_url.is_empty() {
                                img {
                                    src: "{user.profile_url}",
                                    alt: "{user.display_name}",
                                    class: "object-cover object-top w-6 h-6 rounded-full",
                                }
                            } else {
                                div { class: "w-6 h-6 rounded-full bg-neutral-600" }
                            }
                            span { class: "text-sm text-c-secondary truncate", "{user.display_name}" }
                        }

                        // Team entries
                        for team in teams.iter() {
                            Link {
                                class: "flex gap-2 items-center py-1.5 px-2 w-full rounded-md cursor-pointer hover:bg-hover",
                                to: format!("/{}/home", team.username),
                                onclick: move |_| {
                                    open.set(false);
                                },
                                if !team.profile_url.is_empty() {
                                    img {
                                        src: "{team.profile_url}",
                                        alt: "{team.nickname}",
                                        class: "object-cover object-top w-6 h-6 rounded-full",
                                    }
                                } else {
                                    div { class: "w-6 h-6 rounded-full bg-neutral-600" }
                                }
                                span { class: "text-sm text-c-secondary truncate", "{team.nickname}" }
                            }
                        }
                    }

                    // Separator
                    div { class: "my-2 h-px bg-divider" }

                    // Create Team
                    button {
                        class: "py-1.5 px-2 w-full text-sm text-left rounded-md cursor-pointer text-c-secondary hover:bg-hover",
                        onclick: move |_| {
                            open.set(false);
                            popup.open(rsx! {
                                TeamCreationPopup {}
                            });
                            popup.with_title(tr.create_team);
                        },
                        "{tr.create_team}"
                    }

                    // Logout
                    button {
                        class: "py-1.5 px-2 w-full text-sm text-left rounded-md cursor-pointer text-c-secondary hover:bg-hover",
                        onclick: move |_| {
                            open.set(false);
                            spawn(async move {
                                crate::features::auth::services::sign_out(user_ctx).await;
                                nav.push("/");
                            });
                        },
                        "{tr.logout}"
                    }

                    // Delete account (destructive)
                    button {
                        class: "py-1.5 px-2 w-full text-sm text-left text-red-500 rounded-md cursor-pointer hover:bg-hover",
                        onclick: move |_| {
                            open.set(false);
                            popup.open(rsx! {
                                AccountDeletionConfirm {}
                            });
                            popup.with_title(tr.delete_account);
                        },
                        "{tr.delete_account}"
                    }
                }
            }
        }
    }
}

/// Confirmation dialog for irreversible account deletion. On confirm it calls
/// the soft-delete server function, then clears client session state and
/// redirects home.
#[component]
pub fn AccountDeletionConfirm() -> Element {
    let tr: ProfileDropdownTranslate = use_translate();
    let user_ctx = crate::features::auth::hooks::use_user_context();
    let mut popup = use_popup();
    let nav = use_navigator();
    let mut loading = use_signal(|| false);
    let mut error_message: Signal<Option<String>> = use_signal(|| None);

    rsx! {
        div { class: "flex flex-col gap-5 max-w-full w-100",
            div { class: "flex flex-col gap-2",
                h3 { class: "text-lg font-bold text-text-primary", "{tr.delete_account_title}" }
                p { class: "text-sm leading-6 whitespace-pre-wrap text-c-secondary",
                    "{tr.delete_account_warning}"
                }
            }

            if let Some(err) = error_message() {
                p { class: "text-sm text-red-500", "{err}" }
            }

            div { class: "flex flex-col gap-2.5 w-full",
                button {
                    class: "py-2.5 px-4 w-full text-sm font-bold text-white rounded-lg cursor-pointer hover:opacity-90 disabled:opacity-50 disabled:pointer-events-none",
                    style: "background-color: #ef4444;",
                    disabled: loading(),
                    onclick: move |_| async move {
                        error_message.set(None);
                        loading.set(true);
                        match crate::features::auth::controllers::delete_account_handler().await {
                            Ok(_) => {
                                crate::features::auth::services::sign_out(user_ctx).await;
                                nav.push("/");
                            }
                            Err(e) => {
                                crate::error!("delete account failed: {e}");
                                loading.set(false);
                                error_message.set(Some(tr.delete_failed.to_string()));
                            }
                        }
                    },
                    "{tr.confirm_delete}"
                }
                button {
                    class: "py-2.5 px-4 w-full text-sm font-medium rounded-lg border cursor-pointer disabled:opacity-50 disabled:pointer-events-none border-border text-c-secondary hover:bg-hover",
                    disabled: loading(),
                    onclick: move |_| {
                        popup.close();
                    },
                    "{tr.cancel}"
                }
            }
        }
    }
}
