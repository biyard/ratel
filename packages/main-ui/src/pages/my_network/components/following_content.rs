// use dioxus::prelude::*;

// #[derive(Props, PartialEq, Clone)]
// pub struct FollowingContentProps {
//     pub id: i64,
//     pub name: String,
//     pub profile_image: String,
//     pub description: Option<String>,
//     pub is_following: bool,
// }

// #[component]
// pub fn FollowingContent(props: FollowingContentProps) -> Element {
//     let following = use_state(|| props.is_following);

//     let button_text = if *following.get() { "Unfollow" } else { "Follow" };
//     let button_class = if *following.get() {
//         "bg-gray-700 text-white px-3 py-1 rounded"
//     } else {
//         "bg-yellow-400 text-black px-3 py-1 rounded"
//     };

//     rsx! {
//         div { class: "flex items-center justify-between bg-neutral-800 p-4 rounded mb-2",
//             div { class: "flex items-center gap-4",
//                 img { class: "w-10 h-10 rounded-full", src: "{props.profile_image}", alt: "profile" }
//                 div {
//                     div { class: "font-semibold", "{props.name}" }
//                     props.description.as_ref().map(|desc| rsx! {
//                         div { class: "text-sm text-gray-400", "{desc}" }
//                     })
//                 }
//             }
//             button {
//                 class: "{button_class}",
//                 onclick: move |_| following.set(!*following.get()),
//                 "{button_text}"
//             }
//         }
//     }
// }