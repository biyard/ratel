use super::super::components::*;
use super::super::controllers::list_members_handler;
use super::super::*;
use dioxus::prelude::*;
use crate::features::social::pages::group::controllers::{
    add_member_handler, list_groups_handler, remove_member_handler,
};
use crate::features::social::pages::group::dto::{AddMemberRequest, RemoveMemberRequest};

#[component]
pub fn AdminPage(username: String, team_pk: TeamPartition) -> Element {
    let _ = username;
    let mut refresh = use_signal(|| 0u64);

    let member_resource = use_loader(use_reactive((&team_pk,), move |(team_pk,)| {
        let _ = refresh();
        async move {
            Ok::<_, super::super::Error>(
                list_members_handler(team_pk, None, None)
                    .await
                    .map_err(|e| e.to_string()),
            )
        }
    }))?;

    let groups_resource = use_resource(use_reactive((&team_pk,), |(team_pk,)| async move {
        list_groups_handler(team_pk, None)
            .await
            .map(|r| r.items)
            .unwrap_or_default()
    }));

    let data = member_resource.read();
    let members = match data.as_ref() {
        Ok(list) => list.items.clone(),
        Err(_) => vec![],
    };

    let all_groups: Vec<(String, String)> = groups_resource
        .read()
        .as_ref()
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .map(|g| (g.id, g.name))
        .collect();

    let mut removing = use_signal(|| Option::<String>::None);
    let mut changing = use_signal(|| Option::<String>::None);

    let on_remove_from_group: EventHandler<RemovePayload> = Callback::new({
        let team_pk = team_pk.clone();
        let mut removing = removing.clone();
        let mut refresh = refresh.clone();
        move |payload: RemovePayload| {
            let key = format!("{}-{}", payload.member_id, payload.group_id);
            if removing().as_ref() == Some(&key) {
                return;
            }
            removing.set(Some(key.clone()));

            let team_pk = team_pk.clone();
            let mut removing = removing.clone();
            let mut refresh = refresh.clone();
            spawn(async move {
                let result = remove_member_handler(
                    team_pk,
                    payload.group_id,
                    RemoveMemberRequest {
                        user_pks: vec![payload.member_id],
                    },
                )
                .await;

                if result.is_ok() {
                    refresh.set(refresh() + 1);
                }
                removing.set(None);
            });
        }
    });

    let on_change_group: EventHandler<ChangeGroupPayload> = Callback::new({
        let team_pk = team_pk.clone();
        let mut changing = changing.clone();
        let mut refresh = refresh.clone();
        move |payload: ChangeGroupPayload| {
            let key = format!("{}-{}", payload.member_id, payload.from_group_id);
            if changing().as_ref() == Some(&key) {
                return;
            }
            changing.set(Some(key.clone()));

            let team_pk = team_pk.clone();
            let mut changing = changing.clone();
            let mut refresh = refresh.clone();
            spawn(async move {
                let remove_ok = remove_member_handler(
                    team_pk.clone(),
                    payload.from_group_id.clone(),
                    RemoveMemberRequest {
                        user_pks: vec![payload.member_id.clone()],
                    },
                )
                .await
                .is_ok();

                if remove_ok {
                    let add_ok = add_member_handler(
                        team_pk.clone(),
                        payload.to_group_id,
                        AddMemberRequest {
                            user_pks: vec![payload.member_id.clone()],
                        },
                    )
                    .await
                    .is_ok();

                    if add_ok {
                        refresh.set(refresh() + 1);
                    } else {
                        // Rollback: re-add to the original group
                        let _ = add_member_handler(
                            team_pk,
                            payload.from_group_id,
                            AddMemberRequest {
                                user_pks: vec![payload.member_id],
                            },
                        )
                        .await;
                    }
                }
                changing.set(None);
            });
        }
    });

    let rows = members
        .into_iter()
        .filter(|member| {
            !is_blocked_text(&member.display_name) && !is_blocked_text(&member.username)
        })
        .map({
            let on_remove_from_group = on_remove_from_group.clone();
            let on_change_group = on_change_group.clone();
            let removing = removing.clone();
            let changing = changing.clone();
            let all_groups = all_groups.clone();
            move |member| {
                render_member(
                    member,
                    all_groups.clone(),
                    on_remove_from_group.clone(),
                    on_change_group.clone(),
                    removing.clone(),
                    changing.clone(),
                )
            }
        });

    rsx! {
        div { class: "flex flex-col w-full max-w-[1152px] px-4 py-5 gap-[10px] bg-card-bg border border-card-border rounded-lg h-fit",
            for row in rows {
                {row}
            }
        }
    }
}
