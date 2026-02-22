use dioxus::prelude::*;
use ratel_auth::controllers::update_user::{
    update_user_profile_handler, UpdateUserProfileRequest,
};
use ratel_auth::hooks::use_user_context;

#[component]
pub fn Home(username: String) -> Element {
    let user_ctx = use_user_context();
    let user = user_ctx.read().user.clone();

    let Some(user) = user else {
        return rsx! {
            div { class: "flex flex-col items-center justify-center w-full h-full py-10",
                p { class: "text-gray-500", "Please log in to access settings." }
            }
        };
    };

    let mut nickname = use_signal(|| user.display_name.clone());
    let mut profile_url = use_signal(|| user.profile_url.clone());
    let mut description = use_signal(|| user.description.clone());
    let mut saving = use_signal(|| false);
    let mut message = use_signal(|| Option::<String>::None);

    let on_save = move |_| {
        let nick = nickname();
        let profile = profile_url();
        let desc = description();
        spawn(async move {
            saving.set(true);
            message.set(None);
            let result = update_user_profile_handler(UpdateUserProfileRequest {
                nickname: Some(nick),
                profile_url: Some(profile),
                description: Some(desc),
            })
            .await;
            saving.set(false);
            match result {
                Ok(_) => {
                    message.set(Some("Profile updated successfully.".to_string()));
                }
                Err(e) => {
                    message.set(Some(format!("Failed to update profile: {}", e)));
                }
            }
        });
    };

    rsx! {
        div { class: "flex flex-col gap-6 w-full max-w-[600px] py-6",
            h2 { class: "text-xl font-bold text-[var(--text-primary)]", "Profile Settings" }

            div { class: "flex flex-col gap-4",
                div { class: "flex flex-col gap-1",
                    label { class: "text-sm font-medium text-[var(--text-secondary)]", "Username" }
                    input {
                        class: "w-full px-3 py-2 rounded-lg border border-[var(--border-primary)] bg-[var(--bg-secondary)] text-[var(--text-primary)] opacity-50 cursor-not-allowed",
                        r#type: "text",
                        value: "{user.username}",
                        readonly: true,
                    }
                }

                div { class: "flex flex-col gap-1",
                    label { class: "text-sm font-medium text-[var(--text-secondary)]", "Display Name" }
                    input {
                        class: "w-full px-3 py-2 rounded-lg border border-[var(--border-primary)] bg-[var(--bg-secondary)] text-[var(--text-primary)]",
                        r#type: "text",
                        value: "{nickname}",
                        oninput: move |e| nickname.set(e.value()),
                    }
                }

                div { class: "flex flex-col gap-1",
                    label { class: "text-sm font-medium text-[var(--text-secondary)]", "Profile Image URL" }
                    input {
                        class: "w-full px-3 py-2 rounded-lg border border-[var(--border-primary)] bg-[var(--bg-secondary)] text-[var(--text-primary)]",
                        r#type: "text",
                        value: "{profile_url}",
                        oninput: move |e| profile_url.set(e.value()),
                    }
                }

                div { class: "flex flex-col gap-1",
                    label { class: "text-sm font-medium text-[var(--text-secondary)]", "Description" }
                    textarea {
                        class: "w-full px-3 py-2 rounded-lg border border-[var(--border-primary)] bg-[var(--bg-secondary)] text-[var(--text-primary)] min-h-[100px] resize-y",
                        value: "{description}",
                        oninput: move |e| description.set(e.value()),
                    }
                }
            }

            if let Some(msg) = message() {
                div { class: "text-sm text-[var(--text-secondary)]", "{msg}" }
            }

            button {
                class: "w-fit px-6 py-2 rounded-lg bg-[var(--color-primary)] text-white font-medium hover:opacity-90 disabled:opacity-50",
                disabled: saving(),
                onclick: on_save,
                if saving() { "Saving..." } else { "Save" }
            }
        }
    }
}
