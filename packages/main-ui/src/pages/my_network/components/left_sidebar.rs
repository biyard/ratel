// use dioxus::hooks::use_state;
// use dioxus::prelude::*;

// #[component]
// pub fn LeftSidebar() -> Element {
//     let is_user_profile_expanded = use_state(|| true);
//     let is_recent_expanded = use_state(|| true);
//     let is_spaces_expanded = use_state(|| true);

//     rsx! {
//         div { class: "p-6 space-y-6",
//             // User Profile Section
//             div { class: "space-y-4",
//                 div { class: "flex items-center justify-between",
//                     div { class: "flex items-center gap-2",
//                         h2 { class: "text-lg font-semibold", "Hyejin Choi" }
//                         span { class: "text-yellow-400", "👑" }
//                     }
//                     button {
//                         onclick: move |_| is_user_profile_expanded.set(!is_user_profile_expanded.get()),
//                         class: "text-gray-400 hover:text-white transition-colors",
//                         span { class: "w-4 h-4", "⌄" } // Replace with proper icon logic
//                     }
//                 }

//                 if *is_user_profile_expanded.get() {

//                         div { class: "flex items-center gap-3",
//                             div { class: "relative",
//                                 img {
//                                     src: "/placeholder.svg?height=60&width=60",
//                                     alt: "Hyejin Choi",
//                                     class: "w-15 h-15 rounded-full"
//                                 }
//                                 div { class: "absolute -bottom-1 -right-1 w-6 h-6 bg-purple-600 rounded-full flex items-center justify-center",
//                                     span { class: "text-xs", "💎" }
//                                 }
//                             }
//                             div {
//                                 p { class: "text-sm text-gray-400", "Office of Rep" }
//                                 div { class: "flex items-center gap-1",
//                                     span { class: "text-xs", "🇺🇸" }
//                                     span { class: "text-sm text-gray-400", "Oregon, Unite State" }
//                                 }
//                             }
//                         }

//                         div { class: "space-y-2",
//                             div { class: "flex justify-between items-center",
//                                 span { class: "text-sm text-gray-400", "Tier" }
//                                 span { class: "text-sm font-medium", "Diamond 💎" }
//                             }
//                             div { class: "w-full bg-gray-700 rounded-full h-2",
//                                 div { class: "bg-orange-500 h-2 rounded-full w-full" }
//                             }
//                         }

//                 }
//             }

//             // Navigation Menu
//             nav { class: "space-y-2",
//                 SidebarButton { icon: "📄", label: "My Posts" }
//                 SidebarButton { icon: "👤", label: "My Profile" }
//                 SidebarButton { icon: "🔖", label: "Saved Feeds" }
//                 SidebarButton { icon: "💰", label: "Sponsoring" }
//             }

//             // Recent Section
//             div { class: "space-y-3",
//                 div { class: "flex items-center justify-between",
//                     h3 { class: "text-sm font-medium text-gray-400", "📋 Recent" }
//                     button {
//                         onclick: move |_| is_recent_expanded.set(!is_recent_expanded.get()),
//                         class: "text-gray-400 hover:text-white transition-colors",
//                         span { class: "w-4 h-4", "⌃" } // Replace with proper icon logic
//                     }
//                 }
//                 if *is_recent_expanded.get() {

//                         div { class: "space-y-2 text-sm",
//                             div { class: "text-gray-300 hover:text-white cursor-pointer", "Crypto/DAO Treasury Transpre..." }
//                             div { class: "text-gray-300 hover:text-white cursor-pointer", "Crypto/DAO Act Inverstor" }
//                             div { class: "text-gray-300 hover:text-white cursor-pointer", "Crypto/DAO Welcome to Protec..." }
//                         }

//                 }
//             }

//             // Spaces Section
//             div { class: "space-y-3",
//                 div { class: "flex items-center justify-between",
//                     h3 { class: "text-sm font-medium text-gray-400", "🏠 Spaces" }
//                     button {
//                         onclick: move |_| is_spaces_expanded.set(!is_spaces_expanded.get()),
//                         class: "text-gray-400 hover:text-white transition-colors",
//                         span { class: "w-4 h-4", "⌃" } // Replace with proper icon logic
//                     }
//                 }
//                 if *is_spaces_expanded.get() {

//                     div { class: "text-sm text-gray-300 hover:text-white cursor-pointer", "Create Space" }

//                 }
//             }
//         }
//     }
// }

// #[component]
// fn SidebarButton(icon: &'static str, label: &'static str) -> Element {
//     rsx! {
//         button {
//             class: "w-full justify-start text-gray-300 hover:text-white hover:bg-gray-800 flex items-center gap-3",
//             span { "{icon}" }
//             span { "{label}" }
//         }
//     }
// }




use dioxus::prelude::*;

#[component]
pub fn LeftSidebar() -> Element {
    rsx!(
        div { class: "w-1/5 pr-4 space-y-6",
            div { class: "bg-neutral-800 p-4 rounded-lg m-4",
                div { class: "mt-2 font-bold text-lg", "Hyejin Choi 🟡" }
                img { class: "rounded-full w-40 h-40", src: "https://via.placeholder.com/64", alt: "" }

                div {class: "text-white mt-1 flex flex-col", p {"office of Rep"} p {"🇺🇸 Oregon, United State"}}
              
               
                div { class: "text-xs text-gray-400 mt-1 flex justify-between",
                    span { "Tier" }
                    span { "Diamond 🟣" }
                }
                // div { class: "bg-yellow-400 h-1 mt-1 rounded-full" }
                div {class: "bg-neutral-800 h-2 rounded-full overflow-hidden", div { class: "bg-yellow-400 h-full rounded-full", style: "width: 70%;"
                    
                }}
                

            }
            
            // my post section
            div {class:"bg-neutral-800 p-4 rounded-lg m-4",

                ul { class: "space-y-4",
                li { a { href: "#", class: "block hover:text-yellow-400", "📝 My Posts" } }
                li { a { href: "#", class: "block hover:text-yellow-400", "👤 My Profile" } }
                li { a { href: "#", class: "block hover:text-yellow-400", "💾 Saved Feeds" } }
                li { a { href: "#", class: "block hover:text-yellow-400", "💰 Sponsoring" } }
                }

                //My post icon and text
                // div {class:"flex-row", }
                
            }

          
            // Recent section
            div { class:"bg-neutral-800 p-4 rounded-lg mx-4 mt-10",
                h3 { class: "text-gray-400 text-xs uppercase", "Recent" }
                ul { class: "space-y-1 text-sm",
                    li { "Crypto/DAO Treasury Transpre..." }
                    li { "Crypto/DAO Act Investor" }
                    li { "Crypto/DAO Welcome to Protec..." }
                }
            }
            
      
        }
    )
}
