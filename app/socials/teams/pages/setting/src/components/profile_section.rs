use crate::*;
use common::components::FileUploader;

#[component]
pub fn ProfileSection(
    profile_url: String,
    upload_logo_text: String,
    #[props(default)] is_editing: bool,
    on_profile_url_change: EventHandler<String>,
) -> Element {
    let class = if is_editing {
        None
    } else {
        Some("pointer-events-none opacity-60".to_string())
    };

    rsx! {
        div {
            FileUploader {
                on_upload_success: on_profile_url_change,
                class,
                accept: Some("image/*".to_string()),
                upload_endpoint: Some("/api/teams/settings/assets".to_string()),
                if !profile_url.is_empty() {
                    img {
                        src: "{profile_url}",
                        alt: "Team Logo",
                        class: "w-40 h-40 rounded-full object-cover cursor-pointer",
                    }
                } else {
                    button { class: "w-40 h-40 rounded-full bg-c-wg-80 text-sm font-semibold flex items-center justify-center text-c-wg-50",
                        "{upload_logo_text}"
                    }
                }
            }
        }
    }
}
