use crate::components::{MembershipPlanTranslate, MembershipTier};
use crate::*;

#[derive(Clone, Debug)]
pub struct CustomerInfo {
    pub name: String,
    pub card_number: String,
    pub expiry_month: String,
    pub expiry_year: String,
    pub birth_or_biz: String,
    pub card_password: String,
}

fn tier_label(tier: MembershipTier) -> &'static str {
    match tier {
        MembershipTier::Free => "Free",
        MembershipTier::Pro => "Pro",
        MembershipTier::Max => "Max",
        MembershipTier::Vip => "VIP",
        MembershipTier::Enterprise => "Enterprise",
    }
}

#[component]
pub fn MembershipPurchaseModal(
    membership: MembershipTier,
    display_amount: i64,
    customer_name: String,
    on_cancel: EventHandler<()>,
    on_confirm: EventHandler<CustomerInfo>,
) -> Element {
    let tr: MembershipPlanTranslate = use_translate();
    let lang = use_language();
    let currency = match lang() {
        Language::Ko => "₩",
        _ => "$",
    };
    let mut card_number = use_signal(String::new);
    let mut expiry_month = use_signal(String::new);
    let mut expiry_year = use_signal(String::new);
    let mut birth_or_biz = use_signal(String::new);
    let mut card_password = use_signal(String::new);

    let is_valid = !customer_name.trim().is_empty()
        && !card_number.read().trim().is_empty()
        && !expiry_month.read().trim().is_empty()
        && !expiry_year.read().trim().is_empty()
        && !birth_or_biz.read().trim().is_empty()
        && !card_password.read().trim().is_empty();

    let input_class = "flex px-5 w-full min-w-0 h-9 text-base font-light border outline-none md:text-sm disabled:opacity-50 disabled:cursor-not-allowed disabled:pointer-events-none shadow-xs transition-[color,box-shadow] file:text-text-primary file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium selection:bg-primary selection:text-primary-foreground placeholder:text-muted-foreground aria-invalid:ring-destructive/20 aria-invalid:outline aria-invalid:border-c-p-50 bg-input-box-bg border-input-box-border rounded-[10px] py-5.5 text-text-primary dark:bg-input/30 dark:aria-invalid:ring-destructive/40 focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[1px]";

    rsx! {
        div { class: "w-[420px]",
            div { class: "flex flex-col gap-5",
                // Membership Summary
                div { class: "rounded-[10px] bg-card-bg-secondary border border-card-border px-4 py-5",
                    div { class: "flex justify-between items-center",
                        div { class: "flex flex-col gap-1",
                            h4 { class: "text-lg font-semibold text-text-primary",
                                "{tier_label(membership)} {tr.membership_label}"
                            }
                            p { class: "text-sm text-text-secondary", {tr.monthly_subscription} }
                        }
                        h3 { class: "text-xl font-semibold text-primary", "{currency}{display_amount}" }
                    }
                }

                // Customer Information Form
                div { class: "flex flex-col gap-4",
                    div {
                        label { class: "block mb-2 text-sm font-medium text-text-primary",
                            {tr.full_name_label}
                        }
                        input {
                            class: input_class,
                            r#type: "text",
                            value: customer_name.clone(),
                            disabled: true,
                        }
                    }

                    div { class: "pt-4",
                        div { class: "flex flex-col gap-4",
                            div {
                                label { class: "block mb-2 text-sm font-medium text-text-primary",
                                    {tr.card_number_label}
                                }
                                input {
                                    class: input_class,
                                    r#type: "text",
                                    placeholder: tr.card_number_placeholder,
                                    maxlength: 16,
                                    value: card_number,
                                    oninput: move |e| {
                                        let value = e.value().chars().filter(|c| c.is_ascii_digit()).collect::<String>();
                                        card_number.set(value);
                                    },
                                }
                            }

                            div {
                                label { class: "block mb-2 text-sm font-medium text-text-primary",
                                    {tr.expiry_label}
                                }
                                div { class: "flex gap-2",
                                    input {
                                        class: "{input_class} flex-1",
                                        r#type: "text",
                                        placeholder: tr.expiry_month_placeholder,
                                        maxlength: 2,
                                        value: expiry_month,
                                        oninput: move |e| {
                                            let value = e.value().chars().filter(|c| c.is_ascii_digit()).collect::<String>();
                                            expiry_month.set(value);
                                        },
                                    }
                                    input {
                                        class: "{input_class} flex-1",
                                        r#type: "text",
                                        placeholder: tr.expiry_year_placeholder,
                                        maxlength: 2,
                                        value: expiry_year,
                                        oninput: move |e| {
                                            let value = e.value().chars().filter(|c| c.is_ascii_digit()).collect::<String>();
                                            expiry_year.set(value);
                                        },
                                    }
                                }
                            }

                            div {
                                label { class: "block mb-2 text-sm font-medium text-text-primary",
                                    {tr.birth_or_biz_label}
                                }
                                input {
                                    class: input_class,
                                    r#type: "text",
                                    placeholder: tr.birth_or_biz_placeholder,
                                    maxlength: 10,
                                    value: birth_or_biz,
                                    oninput: move |e| {
                                        let value = e.value().chars().filter(|c| c.is_ascii_digit()).collect::<String>();
                                        birth_or_biz.set(value);
                                    },
                                }
                            }

                            div {
                                label { class: "block mb-2 text-sm font-medium text-text-primary",
                                    {tr.card_password_label}
                                }
                                input {
                                    class: "{input_class} w-20",
                                    r#type: "password",
                                    placeholder: tr.card_password_placeholder,
                                    maxlength: 2,
                                    value: card_password,
                                    oninput: move |e| {
                                        let value = e.value().chars().filter(|c| c.is_ascii_digit()).collect::<String>();
                                        card_password.set(value);
                                    },
                                }
                            }
                        }
                    }
                }

                // Footer
                div { class: "flex justify-end gap-4 mt-4",
                    button {
                        class: "px-10 text-base font-bold transition-colors py-[14.5px] bg-cancel-button-bg text-cancel-button-text rounded-[10px] hover:text-cancel-button-text/80",
                        onclick: move |_| {
                            on_cancel.call(());
                        },
                        {tr.cancel_button}
                    }
                    button {
                        class: "px-10 text-base font-bold transition-colors disabled:opacity-50 disabled:cursor-not-allowed py-[14.5px] text-submit-button-text rounded-[10px] bg-submit-button-bg hover:bg-submit-button-bg/80",
                        disabled: !is_valid,
                        onclick: move |_| {
                            if !is_valid {
                                return;
                            }
                            on_confirm
                                .call(CustomerInfo {
                                    name: customer_name.clone(),
                                    card_number: card_number.read().clone(),
                                    expiry_month: expiry_month.read().clone(),
                                    expiry_year: expiry_year.read().clone(),
                                    birth_or_biz: birth_or_biz.read().clone(),
                                    card_password: card_password.read().clone(),
                                });
                        },
                        {tr.confirm_button}
                    }
                }
            }
        }
    }
}
