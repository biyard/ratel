use crate::*;

#[component]
pub fn SpaceLayout(space_id: SpacePartition) -> Element {
    // FIXME: Temporarily set role to Viewer
    let role = SpaceUserRole::Creator;

    let sid = space_id.clone();
    let db_orders = use_server_future(move || {
        let space_id = sid.clone();
        async move { fetch_nav_orders(space_id).await }
    })?
    .value();

    let order_map: Vec<(String, i64)> = db_orders().unwrap_or_default();

    let mut menus: Vec<SpaceNavItem> = vec![
        dashboard::get_nav_item(space_id.clone(), role),
        overview::get_nav_item(space_id.clone(), role),
        actions::get_nav_item(space_id.clone(), role),
        apps::get_nav_item(space_id.clone(), role),
    ]
    .into_iter()
    .filter_map(|s| {
        let item: Result<SpaceNavItem> = s.try_into();
        item.ok()
    })
    .collect();

    // Override orders from DB
    for menu in menus.iter_mut() {
        let page_name = format!("{:?}", menu.label);
        if let Some((_, db_order)) = order_map.iter().find(|(name, _)| name == &page_name) {
            menu.order = *db_order;
        }
    }

    menus.sort_by_key(|m| m.order);

    rsx! {
        div {
            class: "bg-space-bg w-full grid grid-cols-7 h-screen text-font-primary",
            SpaceNav { logo: "https://metadata.ratel.foundation/logos/logo.png", menus  }

            div {
                class: "col-span-6 flex flex-col",
                SpaceTop{}
                div {
                    class: "bg-space-body-bg flex grow rounded-tl-[10px] overflow-auto p-5",
                    Outlet::<Route> {}
                }
            }
        }
    }
}

#[cfg(feature = "server")]
async fn fetch_nav_orders(space_id: SpacePartition) -> Vec<(String, i64)> {
    use crate::models::*;

    let cli = create_dynamo_client();
    let space_pk = Partition::Space(space_id.0);

    let result = SpaceNavItemModel::find_by_space(
        &cli,
        format!("SPACE_NAV#{}", space_pk),
        SpaceNavItemModelQueryOption::builder().limit(50),
    )
    .await;

    match result {
        Ok((items, _)) => items.into_iter().map(|item| (item.page, item.order)).collect(),
        Err(_) => Vec::new(),
    }
}

#[cfg(not(feature = "server"))]
async fn fetch_nav_orders(_space_id: SpacePartition) -> Vec<(String, i64)> {
    Vec::new()
}

#[cfg(feature = "server")]
fn create_dynamo_client() -> aws_sdk_dynamodb::Client {
    use aws_sdk_dynamodb::{Client, Config, config::Credentials};

    let endpoint = std::env::var("DYNAMO_ENDPOINT").ok().and_then(|v| {
        if v.is_empty() || v.eq_ignore_ascii_case("none") {
            None
        } else {
            Some(v)
        }
    });

    let region = std::env::var("AWS_REGION").unwrap_or_else(|_| "ap-northeast-2".to_string());
    let access_key = std::env::var("AWS_ACCESS_KEY_ID").unwrap_or_default();
    let secret_key = std::env::var("AWS_SECRET_ACCESS_KEY").unwrap_or_default();

    let mut builder = Config::builder()
        .region(aws_sdk_dynamodb::config::Region::new(region))
        .behavior_version_latest();

    if !access_key.is_empty() && !secret_key.is_empty() {
        builder = builder.credentials_provider(Credentials::new(
            access_key,
            secret_key,
            None,
            None,
            "space-shell",
        ));
    }

    if let Some(endpoint) = endpoint {
        builder = builder.endpoint_url(endpoint);
    }

    Client::from_conf(builder.build())
}
