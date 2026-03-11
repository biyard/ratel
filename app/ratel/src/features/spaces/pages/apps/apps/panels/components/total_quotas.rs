use crate::features::spaces::pages::apps::apps::panels::*;

translate! {
    TotalQuotasTranslate;

    total_quotas: {
        en: "Total quotas",
        ko: "총 쿼터",
    },
}

#[component]
pub fn TotalQuotas(space_id: ReadSignal<SpacePartition>, quota: i64) -> Element {
    let tr: TotalQuotasTranslate = use_translate();
    let space_ctx = use_space_context();
    let mut toast = use_toast();
    let mut total_quota_input = use_signal(|| quota.to_string());
    let mut synced_quota = use_signal(|| quota);

    use_effect(move || {
        if synced_quota() != quota {
            synced_quota.set(quota);
            total_quota_input.set(quota.to_string());
        }
    });

    let on_confirm = {
        move |_| {
            let next_quota = total_quota_input().parse::<i64>().unwrap_or_default();
            let mut space_ctx = space_ctx;
            let mut toast = toast;
            spawn(async move {
                match update_space(space_id(), UpdateSpaceRequest::Quota { quotas: next_quota })
                    .await
                {
                    Ok(_) => space_ctx.space.restart(),
                    Err(err) => {
                        error!("Failed to update panel quota: {:?}", err);
                        toast.error(err);
                    }
                }
            });
        }
    };
    let on_blur = {
        move |_| {
            let next_quota = total_quota_input().parse::<i64>().unwrap_or_default();
            let mut space_ctx = space_ctx;
            let mut toast = toast;
            spawn(async move {
                match update_space(space_id(), UpdateSpaceRequest::Quota { quotas: next_quota })
                    .await
                {
                    Ok(_) => space_ctx.space.restart(),
                    Err(err) => {
                        error!("Failed to update panel quota: {:?}", err);
                        toast.error(err);
                    }
                }
            });
        }
    };

    rsx! {
        div { class: "flex items-center gap-5 shrink-0",
            div { class: "text-sm font-medium text-text-primary whitespace-nowrap",
                {tr.total_quotas}
            }
            Input {
                class: "w-20 h-9 !px-3 text-center text-sm font-semibold".to_string(),
                value: total_quota_input(),
                oninput: move |evt: Event<FormData>| {
                    let digits = evt
                        .value()
                        .chars()
                        .filter(|ch| ch.is_ascii_digit())
                        .collect::<String>();
                    total_quota_input.set(digits);
                },
                onconfirm: on_confirm,
                onblur: on_blur,
            }
        }
    }
}
