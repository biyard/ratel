use super::super::*;
use crate::common::components::FileUploader;

#[component]
pub fn ProfileSection(
    profile_url: String,
    upload_logo_text: String,
    #[props(default)] is_editing: bool,
    on_profile_url_change: EventHandler<String>,
) -> Element {
    rsx! {
        div {
            FileUploader {
                on_upload_success: on_profile_url_change,
                class: if !is_editing { "pointer-events-none opacity-60" },
                accept: "image/*",
                if !profile_url.is_empty() {
                    img {
                        src: profile_url,
                        alt: "Team Logo",
                        class: "object-cover w-40 h-40 rounded-full cursor-pointer",
                    }
                } else {
                    button { class: "flex justify-center items-center w-40 h-40 text-sm font-semibold rounded-full bg-c-wg-80 text-c-wg-50",
                        {upload_logo_text}
                    }
                }
            }
        }
    }
}
