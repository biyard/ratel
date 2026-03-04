mod i18n;

pub use i18n::*;

use crate::components::MembershipTier;
use crate::services::portone::VerifiedCustomer;
use crate::*;
use common::components::Card;

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
    customer: VerifiedCustomer,
    on_confirm: EventHandler<CustomerInfo>,
) -> Element {
    let mut popup = use_popup();
    let tr: MembershipPurchaseTranslate = use_translate();
    let lang = use_language();
    let currency = use_memo(move || match lang() {
        Language::Ko => "₩",
        _ => "$",
    });
    let amount_text = use_memo(move || format!("{}{}", currency(), display_amount));
    let customer_name = customer.name.clone();

    let customer_birth_date = customer.birth_date.clone();
    let birth_or_biz_value = use_memo(move || {
        let raw_birth = customer_birth_date.as_str();
        let birth_value = raw_birth.replace('-', "");
        let is_business = raw_birth
            .split('-')
            .next()
            .map(|value| value.len() == 3)
            .unwrap_or(false);

        if is_business {
            birth_value.chars().take(10).collect::<String>()
        } else {
            birth_value.chars().skip(2).take(6).collect::<String>()
        }
    });

    let mut card_number = use_signal(|| "".to_string());
    let mut expiry_month = use_signal(|| "".to_string());
    let mut expiry_year = use_signal(|| "".to_string());
    let mut birth_or_biz = use_signal(|| "".to_string());
    let mut card_password = use_signal(|| "".to_string());

    let customer_name_for_validation = customer.name.clone();
    let is_valid = use_memo(move || {
        !customer_name_for_validation.trim().is_empty()
            && !card_number.read().trim().is_empty()
            && !expiry_month.read().trim().is_empty()
            && !expiry_year.read().trim().is_empty()
            && !birth_or_biz.read().trim().is_empty()
            && !card_password.read().trim().is_empty()
    });

    rsx! {
        div { class: "w-full max-w-[420px]",
            div { class: "flex flex-col gap-5",
                // Membership Summary
                Card {
                    div { class: "flex justify-between items-center",
                        div { class: "flex flex-col gap-1",
                            h4 { class: "text-lg font-semibold text-text-primary",
                                {tier_label(membership)}
                                " "
                                {tr.membership_label}
                            }
                            p { class: "text-sm text-text-secondary", {tr.monthly_subscription} }
                        }
                        h3 { class: "text-xl font-semibold text-primary", {amount_text()} }
                    }
                }

                // Customer Information Form
                div { class: "flex flex-col gap-4",
                    div {
                        label { class: "block mb-2 text-sm font-medium text-text-primary",
                            {tr.full_name_label}
                        }
                        Input { value: customer_name.clone(), disabled: true }
                    }

                    div { class: "pt-4",
                        div { class: "flex flex-col gap-4",
                            div {
                                label { class: "block mb-2 text-sm font-medium text-text-primary",
                                    {tr.card_number_label}
                                }
                                Input {
                                    placeholder: tr.card_number_placeholder,
                                    maxlength: 16,
                                    value: card_number(),
                                    oninput: move |e: Event<FormData>| {
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
                                    Input {
                                        class: "flex-1",
                                        placeholder: tr.expiry_month_placeholder,
                                        maxlength: 2,
                                        value: expiry_month(),
                                        oninput: move |e: Event<FormData>| {
                                            let value = e.value().chars().filter(|c| c.is_ascii_digit()).collect::<String>();
                                            expiry_month.set(value);
                                        },
                                    }
                                    Input {
                                        class: "flex-1",
                                        placeholder: tr.expiry_year_placeholder,
                                        maxlength: 2,
                                        value: expiry_year(),
                                        oninput: move |e: Event<FormData>| {
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
                                Input {
                                    placeholder: tr.birth_or_biz_placeholder,
                                    maxlength: 10,
                                    value: birth_or_biz(),
                                    oninput: move |e: Event<FormData>| {
                                        let value = e.value().chars().filter(|c| c.is_ascii_digit()).collect::<String>();
                                        birth_or_biz.set(value);
                                    },
                                }
                            }

                            div {
                                label { class: "block mb-2 text-sm font-medium text-text-primary",
                                    {tr.card_password_label}
                                }
                                Input {
                                    class: "w-20",
                                    r#type: "password",
                                    placeholder: tr.card_password_placeholder,
                                    maxlength: 2,
                                    value: card_password(),
                                    oninput: move |e: Event<FormData>| {
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
                    Button {
                        style: ButtonStyle::Outline,
                        onclick: move |_| {
                            popup.close();
                        },
                        {tr.cancel_button}
                    }
                    Button {
                        style: ButtonStyle::Primary,
                        disabled: !is_valid(),
                        onclick: move |_| {
                            if !*is_valid.read() {
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
