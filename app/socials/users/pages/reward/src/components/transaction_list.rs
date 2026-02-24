use crate::{
    components::transaction_item, dto::PointTransactionResponse, views::RewardsPageTranslate, *,
};

pub fn transaction_list(
    tr: &RewardsPageTranslate,
    transactions: &[PointTransactionResponse],
    is_loading: bool,
    has_error: bool,
) -> Element {
    if is_loading {
        return rsx! {
            div { class: "py-8 text-center text-text-primary", "{tr.loading}" }
        };
    }

    if has_error {
        return rsx! {
            div { class: "py-8 text-center text-destructive", "{tr.error}" }
        };
    }

    if transactions.is_empty() {
        return rsx! {
            div { class: "py-16 text-center",
                h3 { class: "text-lg font-semibold text-white mb-2", "{tr.empty}" }
                p { class: "text-sm text-text-primary", "{tr.empty_description}" }
            }
        };
    }

    rsx! {
        div { class: "flex flex-col gap-0",
            for (idx , item) in transactions.iter().enumerate() {
                {transaction_item(tr, item, idx)}
            }
        }
    }
}
