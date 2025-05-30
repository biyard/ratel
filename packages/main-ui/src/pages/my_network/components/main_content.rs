







// use dioxus::prelude::*;
// use dioxus::hooks::use_resource;
// use serde::Deserialize;

// mod following_content;
// use following_content::FollowingContent;

// #[derive(Deserialize, Clone, PartialEq)]
// struct Follower {
//     id: i64,
//     profile_image: String,
//     title: String,
//     description: Option<String>,
//     followed: bool,
//     // ...other fields as needed
// }

// #[component]
// pub fn MainContent() -> Element {
//     // Fetch followers from the API
//     let followers = use_resource(|| async {
//         // Use gloo-net for WASM or reqwest for server
//         gloo_net::http::Request::get("/api/v1/followers")
//             .send()
//             .await
//             .ok()
//             .and_then(|resp| resp.json::<Vec<Follower>>().await.ok())
//             .unwrap_or_default()
//     });

//     rsx! {
//         div { class: "w-3/5 space-y-6",
//             div {
//                 div { class: "text-lg font-semibold mb-2", "{followers.read().len()} Following" }
//                 div { class: "space-y-3",
//                     match followers.read().is_empty() {
//                         true => rsx!(div { "No following users found." }),
//                         false => rsx! {
//                             followers.read().iter().map(|user| rsx!(
//                                 FollowingContent {
//                                     id: user.id,
//                                     name: user.title.clone(),
//                                     profile_image: user.profile_image.clone(),
//                                     description: user.description.clone(),
//                                     is_following: user.followed,
//                                 }
//                             ))
//                         }
//                     }
//                 }
//             }
//             // ...your suggested accounts section here...
//         }
//     }
// }



use dioxus::prelude::*;

#[component]
pub fn MainContent() -> Element {
    rsx!(
        div { class: "w-3/5 space-y-6",

            div {
                div { class: "text-lg font-semibold mb-2", "2 Following" }
                div { class: "space-y-3",
                    {(0..4).map(|_| rsx!(
                        div { class: "bg-neutral-800 p-4 rounded flex justify-between items-center p-4",
                            div { class: "flex items-center space-x-4",
                                div { class: "w-40 h-40 rounded-full bg-neutral-700" }
                                div {
                                    div { class: "font-semibold", "User name" }
                                    div { class: "text-sm text-gray-400", "Candidate for State Senate, NY, Reform Alliance" }
                                }
                            }
                            button { class: "bg-neutral-700 text-white px-4 py text-sm rounded-full", "Following" }
                        }
                    ))}
                }
            }

            div {
                div { class: "text-lg font-semibold mt-6 mb-2", "Suggested account" }
                div { class: "space-y-3",
                    {(0..2).map(|_| rsx!(
                        div { class: "bg-neutral-800 p-4 rounded flex justify-between items-center",
                            div { class: "flex items-start space-x-4",
                                div { class: "w-40 h-40 rounded-full bg-neutral-700" }
                                div {
                                    div { class: "font-semibold", "User name" }
                                    div { class: "text-sm text-white", "@heythisisyourid" }
                                    div { class: "text-sm text-gray-400 mt-1 max-w-2xl",
                                        "Candidate for State Senate, NY, Reform Alliance\nA small but certain changemaker for everyday lives. Change starts close to home."
                                    }
                                }
                            }
                            // button { class: "bg-white text-black px-4 py-1 text-sm rounded-full",  "+ Follow" }
                            button {class: "bg-white text-black px-4 py-1 text-sm", span {"+"} span {"Follow"}}
                        }
                    ))}
                }
            }
        }
    )
}

