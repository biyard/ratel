use super::super::components::*;
use super::super::controllers::{list_members_handler, remove_team_member_handler};
use super::super::dto::RemoveMemberRequest;
use super::super::*;
use dioxus::prelude::*;

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

    let data = member_resource.read();
    let members = match data.as_ref() {
        Ok(list) => list.items.clone(),
        Err(_) => vec![],
    };

    let mut removing = use_signal(|| Option::<String>::None);

    let on_remove: EventHandler<RemovePayload> = Callback::new({
        let team_pk = team_pk.clone();
        let mut removing = removing.clone();
        let mut refresh = refresh.clone();
        move |payload: RemovePayload| {
            let key = format!("remove:{}", payload.member_id);
            if removing().as_ref() == Some(&key) {
                return;
            }
            removing.set(Some(key));

            let team_pk = team_pk.clone();
            let mut removing = removing.clone();
            let mut refresh = refresh.clone();
            spawn(async move {
                let result = remove_team_member_handler(
                    team_pk,
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
            let on_remove = on_remove.clone();
            let removing = removing.clone();
            move |member| render_member(member, on_remove.clone(), removing.clone())
        });

    rsx! {
        div { class: "flex flex-col w-full max-w-[1152px] px-4 py-5 gap-[10px] bg-card-bg border border-card-border rounded-lg h-fit",
            for row in rows {
                {row}
            }
        }
    }
}
