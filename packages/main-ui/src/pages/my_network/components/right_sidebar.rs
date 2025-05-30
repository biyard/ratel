

use dioxus::prelude::*;

#[component]
pub fn RightSidebar() -> Element {
    rsx!(
        div { class: "w-1/5 pl-4 space-y-6",

            div { class:"bg-neutral-800 p-4 rounded-lg m-4",
                h3 { class: "text-lg text-gray-400 mb-2 font-semibold", "Hot Promotion" }
                div { class: "bg-neutral-800 p-3 rounded text-sm",
                    img { class: "w-40 h-40 rounded", src: "https://via.placeholder.com/150", alt: "" }

                    div { class: "font-semibold mt-2", "Crypto promotion" }
                    div { class: "text-gray-400", "A new research on engagement of crypto ..." }
                    a { href: "#", class: "text-yellow-400 text-xs mt-1 block", "View all →" }
                }
            }

            div { class:"bg-neutral-800 p-4 rounded-lg m-4",
                h3 { class: "text-lg text-gray-400 mb-2", "News" }
                div { class: "space-y-2 text-sm",
                    // div { "Legislative social\nRatel has launched legislative social to engage industry-friendly legislative ..." }
                    
                    // Lagislative candidate
                    div { class:"flex flex-col  border-t border-[#9CA3AF]", p { class:"text-lg", "Legislative social"} p {class:"text-sm text-gray-400", "Ratel has launched legislative social to engage industry-friendly legislative ..."}  }


                    // Presedential candidate
                    div { class:"flex flex-col  border-t border-[#9CA3AF] ", p { class:"text-lg", "Presidential candidates"} p {class:"text-sm text-gray-400", "Ratel has started to like election pledges on crypto industry of each presidential ..."}  }


                    // New promotions
                    div { class:"flex flex-col  border-t border-[#9CA3AF]", p { class:"text-lg", "News Promotion"} p {class:"text-sm text-gray-400", "Ratel engages and supports marketing for industrial promotions which helps ..."}  }

                    


                
                    a { href: "#", class: "text-gray-400 text-xs block", "View all -->" }
                }
            }

         
        }
    )
}
