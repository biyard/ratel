use super::*;

mod anonymous_setting;
mod invite_participant;
mod join_anytime_setting;
mod space_logo_setting;
mod space_visibility_setting;
mod start_time_setting;
use invite_participant::*;
mod administrators;
use administrators::*;
use anonymous_setting::*;
use join_anytime_setting::*;
use space_logo_setting::*;
use space_visibility_setting::*;
use start_time_setting::*;

const DEFAULT_PROFILE_IMAGE: &str = "https://metadata.ratel.foundation/ratel/default-profile.png";

fn normalize_email_inputs(raw: &str) -> Result<Vec<String>> {
    let emails: Vec<String> = raw
        .split(',')
        .map(|value| value.trim().to_ascii_lowercase())
        .filter(|value| !value.is_empty())
        .collect();

    if emails.is_empty() {
        return Err(Error::InvalidEmail);
    }

    let mut normalized = Vec::new();
    for email in emails {
        if !email.contains('@') {
            return Err(Error::InvalidEmail);
        }

        if !normalized.iter().any(|value| value == &email) {
            normalized.push(email);
        }
    }

    Ok(normalized)
}

#[component]
pub fn SpaceGeneralAppPage(space_id: ReadSignal<SpacePartition>) -> Element {
    let navigator = use_navigator();
    let mut toast = use_toast();
    let tr: GeneralTranslate = use_translate();

    let mut loading = use_signal(|| false);

    rsx! {
        div { class: "flex overflow-visible flex-col gap-5 self-start pb-6 min-w-0 shrink-0 w-full max-tablet:gap-4 text-web-font-primary",
            h3 { class: "font-bold font-raleway text-[24px]/[28px] tracking-[-0.24px] text-web-font-primary",
                {tr.space_setting}
            }
            SpaceLogoSetting {}

            StartTimeSetting {}

            SpaceVisibilitySetting {}

            InviteParticipant {}

            AnonymousSetting {}

            JoinAnytimeSetting {}

            Administrators {}

            div { class: "flex justify-end pt-5 w-full max-tablet:justify-stretch",
                Button {
                    class: "border w-fit max-tablet:w-full border-web-error !bg-transparent !text-web-error hover:!bg-transparent hover:!border-web-error hover:!text-web-error disabled:!bg-transparent disabled:border-web-error/40 disabled:!text-web-error/40",
                    style: ButtonStyle::Text,
                    loading: loading(),
                    onclick: move |_| async move {
                        if loading() {
                            return;
                        }
                        loading.set(true);
                        let result = delete_space(space_id()).await;
                        loading.set(false);
                        match result {
                            Ok(_) => {
                                toast.info(tr.space_deleted_successfully);
                                navigator.push(Route::Index {});
                            }
                            Err(err) => {
                                toast.error(err);
                            }
                        }
                    },
                    {tr.delete_space}
                }
            }
        }
    }
}
