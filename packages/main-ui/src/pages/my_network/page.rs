use super::*;
use controller::*;
use bdk::prelude::*;
use i18n::*;

#[component]
pub fn MyNetworkPage(#[props(default = Language::En)] lang: Language) -> Element {
    let ctrl = use_signal(|| FollowingController::new());
    let tr: MyNetworkTranslate = translate(&lang);

    let load_users = use_future(|| async move {
        ctrl.write().await.load_following_data().await;
    });

    rsx! {
        by_components::meta::MetaPage { title: tr.title }

        div {
            id: "my-network",
            class: "flex flex-col lg:flex-row min-h-screen bg-neutral-950 text-white",

            Sidebar{}

            div {
                class: "flex-1 p-4 lg:px-6",

                h1 {
                    class: "text-2xl font-bold mb-4",
                    "{tr.title}"
                }

                Feed { controller: ctrl.clone() }
            }

            AnimatedCard {
                div {
                    class: "hidden lg:block w-full max-w-sm",
                    NewsRightPanel {}
                }
            }
        }
    }
}
