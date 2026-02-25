use crate::components::*;
use crate::controllers::list_members_handler;
use crate::*;
use dioxus::prelude::*;
use ratel_team_group::controllers::remove_member_handler;
use ratel_team_group::dto::RemoveMemberRequest;

#[component]
pub fn AdminPage(teamname: String, team_pk: TeamPartition) -> Element {
    let _ = teamname;
    let mut refresh = use_signal(|| 0u64);
    let team_pk_clone = team_pk.clone();
    let refresh_clone = refresh.clone();
    let member_resource = use_server_future(move || {
        let _ = refresh_clone();
        let team_pk = team_pk_clone.clone();
        async move { list_members_handler(team_pk, None, None).await }
    })?;

    let resolved = member_resource.suspend()?;
    let data = resolved.read();
    let members = match data.as_ref() {
        Ok(list) => list.items.clone(),
        Err(_) => vec![],
    };

    let mut removing = use_signal(|| Option::<String>::None);

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

    let rows = members
        .into_iter()
        .filter(|member| {
            !is_blocked_text(&member.display_name) && !is_blocked_text(&member.username)
        })
        .map({
            let on_remove_from_group = on_remove_from_group.clone();
            let removing = removing.clone();
            move |member| render_member(member, on_remove_from_group.clone(), removing.clone())
        });

    rsx! {
        div { class: "flex flex-col w-full max-w-[1152px] px-4 py-5 gap-[10px] bg-card-bg border border-card-border rounded-lg h-fit",
            for row in rows {
                {row}
            }
        }
    }
}
