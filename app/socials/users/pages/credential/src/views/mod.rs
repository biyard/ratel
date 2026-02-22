use crate::controllers::get_credentials::{get_credentials_handler, CredentialResponse};
use dioxus::prelude::*;

#[component]
pub fn Home(username: String) -> Element {
    let resource = use_server_future(move || async move {
        get_credentials_handler().await
    })?;

    let resolved = resource.suspend()?;
    let data = resolved.read();

    let credential = match data.as_ref() {
        Ok(data) => data.clone(),
        Err(_) => CredentialResponse::default(),
    };

    let has_any = credential.age.is_some()
        || credential.gender.is_some()
        || credential.university.is_some();

    if !has_any {
        return rsx! {
            div { class: "flex flex-col gap-4 w-full py-6",
                h2 { class: "text-xl font-bold text-[var(--text-primary)]", "Verified Credentials" }
                div { class: "flex flex-row justify-start items-center w-full text-base font-medium text-gray-500 border border-gray-500 h-fit px-[16px] py-[20px] rounded-[8px]",
                    "No verified credentials found"
                }
            }
        };
    }

    rsx! {
        div { class: "flex flex-col gap-6 w-full py-6",
            h2 { class: "text-xl font-bold text-[var(--text-primary)]", "Verified Credentials" }
            div { class: "flex flex-col gap-3",
                if let Some(age) = credential.age {
                    CredentialItem { label: "Age", value: format!("{}", age) }
                }
                if let Some(gender) = &credential.gender {
                    CredentialItem { label: "Gender", value: gender.clone() }
                }
                if let Some(university) = &credential.university {
                    CredentialItem { label: "University", value: university.clone() }
                }
            }
        }
    }
}

#[component]
fn CredentialItem(label: String, value: String) -> Element {
    rsx! {
        div { class: "flex flex-row items-center gap-4 p-4 rounded-lg border border-[var(--border-primary)]",
            div { class: "flex flex-col",
                p { class: "text-sm text-[var(--text-secondary)]", "{label}" }
                p { class: "text-base font-medium text-[var(--text-primary)]", "{value}" }
            }
        }
    }
}
