use crate::components::{DaoInfoCard, DaoRegistrationCard, RegisterDaoPopup};
use crate::dto::TeamDao;
use crate::models::{DaoWalletError, create_dao};
use crate::views::TeamDaoTranslate;
use crate::*;
use dioxus::prelude::*;
use ratel_post::types::{TeamGroupPermission, TeamGroupPermissions};
use ratel_team_setting::controllers::{UpdateTeamRequest, update_team_handler};

#[component]
pub fn AdminPage(teamname: String, context: TeamDao) -> Element {
    let tr: TeamDaoTranslate = use_translate();
    let mut toast = use_toast();

    let mut team_state = use_signal(|| context.team.clone());
    let eligible_admins = context.eligible_admins.clone();
    let eligible_admins_count = eligible_admins.len();

    let permissions: TeamGroupPermissions = context.permissions.into();
    let is_admin = permissions.contains(TeamGroupPermission::TeamAdmin);
    let can_register = eligible_admins_count >= 3 && is_admin;

    let mut is_popup_open = use_signal(|| false);
    let mut is_registering = use_signal(|| false);

    let conf = crate::config::get();
    let network = match conf.common.env {
        Environment::Production => "mainnet",
        _ => "testnet",
    };

    let explorer_base = conf.block_explorer_url.trim().to_string();
    let block_explorer_url = team_state().dao_address.as_ref().and_then(|addr| {
        if explorer_base.is_empty() {
            None
        } else if addr.is_empty() {
            None
        } else {
            Some(format!(
                "{}/account/{}",
                explorer_base.trim_end_matches('/'),
                addr
            ))
        }
    });

    let on_open_registration_popup = {
        let mut is_popup_open = is_popup_open.clone();
        let mut toast = toast;
        move |_| {
            if !can_register {
                toast.error(common::Error::InsufficientAdmins);
                return;
            }
            is_popup_open.set(true);
        }
    };

    let on_close_popup = {
        let mut is_popup_open = is_popup_open.clone();
        let is_registering = is_registering.clone();
        move |_| {
            if !is_registering() {
                is_popup_open.set(false);
            }
        }
    };

    let on_register = {
        let mut is_registering = is_registering.clone();
        let mut is_popup_open = is_popup_open.clone();
        let mut team_state = team_state.clone();
        let teamname = teamname.clone();
        let rpc_url = conf.rpc_url.clone();
        let block_explorer_url = conf.block_explorer_url.clone();
        let network = network.to_string();
        let mut toast = toast;
        move |selected_addresses: Vec<String>| {
            if selected_addresses.len() < 3 {
                toast.error(common::Error::InsufficientAdmins);
                return;
            }

            if is_registering() {
                return;
            }

            let teamname = teamname.clone();
            let rpc_url = rpc_url.clone();
            let block_explorer_url = block_explorer_url.clone();
            let network = network.clone();
            let mut is_registering = is_registering.clone();
            let mut is_popup_open = is_popup_open.clone();
            let mut team_state = team_state.clone();
            let mut toast = toast;
            spawn(async move {
                is_registering.set(true);
                toast.info("Connecting to Kaia network...");
                toast.info("Creating DAO on blockchain...");

                let create_result =
                    create_dao(selected_addresses, &network, &rpc_url, &block_explorer_url).await;

                let dao_result = match create_result {
                    Ok(result) => result,
                    Err(err) => {
                        handle_wallet_error(&mut toast, err);
                        is_registering.set(false);
                        return;
                    }
                };

                toast.info("Saving DAO address...");
                let update_result = update_team_handler(
                    teamname,
                    UpdateTeamRequest {
                        nickname: None,
                        description: None,
                        profile_url: None,
                        dao_address: Some(dao_result.dao_address.clone()),
                    },
                )
                .await;

                match update_result {
                    Ok(_) => {
                        let mut updated = team_state();
                        updated.dao_address = Some(dao_result.dao_address.clone());
                        team_state.set(updated);

                        let tx_short = dao_result
                            .transaction_hash
                            .chars()
                            .take(10)
                            .collect::<String>();
                        toast.info(format!(
                            "DAO registered successfully! Transaction: {}...",
                            tx_short
                        ));
                        is_popup_open.set(false);
                    }
                    Err(err) => {
                        toast.error(common::Error::Unknown(format!("Failed to register DAO: {}", err)));
                    }
                }

                is_registering.set(false);
            });
        }
    };

    rsx! {
        div { class: "flex flex-col w-full max-w-[1152px] gap-5 p-6",
            div { class: "mb-4",
                h1 { class: "text-3xl font-bold text-text-primary mb-2", "{tr.dao_title}" }
                p { class: "text-text-secondary", "{tr.dao_description}" }
            }

            if let Some(dao_address) = team_state()
                .dao_address
                .clone()
                .filter(|addr| !addr.is_empty())
            {
                DaoInfoCard { dao_address, explorer_url: block_explorer_url }
            } else {
                DaoRegistrationCard {
                    on_register: on_open_registration_popup,
                    eligible_count: eligible_admins_count,
                    min_required: 3,
                    can_register,
                }
            }

            if is_popup_open() {
                RegisterDaoPopup {
                    eligible_admins: eligible_admins.clone(),
                    on_register,
                    on_cancel: on_close_popup,
                    is_registering: is_registering(),
                }
            }
        }
    }
}

fn handle_wallet_error(toast: &mut ToastService, err: DaoWalletError) {
    match err.code.as_deref() {
        Some("USER_REJECTED") => {
            toast.error(common::Error::TransactionRejected);
        }
        Some("METAMASK_NOT_INSTALLED") => {
            toast.error(common::Error::MetamaskNotInstalled);
        }
        Some(code) => {
            toast.error(common::Error::WalletError(format!("{} ({})", err.message, code)));
        }
        None => {
            toast.error(common::Error::WalletError(err.message));
        }
    }
}
