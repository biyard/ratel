use super::*;
use bdk::prelude::*;
use components::*;
use controller::*;
use dto::*;
use i18n::*;

#[component]
pub fn MyProfilePage(#[props(default = Language::En)] lang: Language) -> Element {
    let mut ctrl = Controller::new(lang)?;
    let tr: MyProfileTranslate = translate(&lang);

    rsx! {
        by_components::meta::MetaPage { title: tr.title, description: tr.description }

        div {
            id: "my-profile",
            class: "w-full max-w-desktop min-h-screen flex flex-col !justify-start gap-72 py-150 max-tablet:!px-30 max-tablet:!overflow-y-scroll max-tablet:!pt-40 px-10",
            div { class: "w-full flex flex-row gap-100",
                div { class: "min-w-150 flex flex-col items-center justify-center",
                    img {
                        class: "w-150 h-150 rounded-full object-cover",
                        src: ctrl.profile_url(),
                    }
                }

                div { class: "grow-1 flex flex-col gap-24",
                    label { class: "w-full flex flex-row gap-24 items-center",
                        span { class: "w-100", {tr.label_name} }

                        input {
                            name: "name",
                            class: "w-full max-w-500 max-tablet:!max-w-full",
                            value: ctrl.name(),
                            oninput: move |e| ctrl.name.set(e.value()),
                        }
                    }

                    label { class: "w-full flex flex-row gap-24 items-center justify-between",
                        span { class: "w-100", {tr.label_membership} }

                        button {
                            class: "btn secondary sm",
                            onclick: move |_| ctrl.upgrade_membership(),
                            {tr.btn_upgrade}
                        }

                    }

                    div { class: "w-full grid grid-cols-4 gap-10",
                        for membership in Membership::VARIANTS {
                            MembershipCard { selected: false, membership: *membership }
                        }
                    }


                    label { class: "w-full flex flex-row gap-24 items-center",
                        span { class: "w-100", {tr.label_email} }
                        input {
                            name: "email",
                            disabled: true,
                            class: "w-full max-w-500 max-tablet:!max-w-full",
                            value: ctrl.email(),
                        }
                    }

                    label { class: "w-full flex flex-row gap-24 items-center",
                        {tr.label_agree}
                        "{ctrl.aggree_getting_info}"
                    }
                }
            }
        } // end of this page
    }
}
