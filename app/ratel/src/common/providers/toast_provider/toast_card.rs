use dioxus::prelude::*;

use super::{use_toast, ToastItem, ToastLevel};

#[component]
pub fn ToastCard(toast: ToastItem) -> Element {
    let mut toast_svc = use_toast();
    let id = toast.id;
    let dismissing = toast.dismissing;
    let mut drag_start_x: Signal<Option<f64>> = use_signal(|| None);
    let mut drag_offset: Signal<f64> = use_signal(|| 0.0);
    let mut mounted = use_signal(|| false);

    use_effect(move || {
        mounted.set(true);
    });

    // Auto-dismiss after 5 seconds
    let _auto_dismiss = use_future(move || {
        let mut svc = toast_svc;
        async move {
            #[cfg(feature = "web")]
            {
                gloo_timers::future::sleep(std::time::Duration::from_secs(5)).await;
                svc.dismiss(id);
                gloo_timers::future::sleep(std::time::Duration::from_millis(300)).await;
                svc.remove(id);
            }
            #[cfg(feature = "server")]
            {
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                svc.dismiss(id);
                tokio::time::sleep(std::time::Duration::from_millis(300)).await;
                svc.remove(id);
            }
        }
    });

    let border_color = match toast.level {
        ToastLevel::Info => "border-l-blue-500",
        ToastLevel::Warn => "border-l-yellow-500",
        ToastLevel::Error => "border-l-red-500",
    };

    let slide_class = if dismissing {
        "translate-x-full opacity-0"
    } else if *mounted.read() {
        "translate-x-0 opacity-100"
    } else {
        "translate-x-full opacity-0"
    };

    let offset = *drag_offset.read();
    let transform_style = if offset < 0.0 {
        format!("transform: translateX({}px);", offset)
    } else {
        String::new()
    };

    let link = toast.link.clone();
    let has_link = link.is_some();

    rsx! {
        div {
            class: "flex gap-3 items-center p-4 rounded-lg border border-l-4 shadow-lg transition-all duration-300 cursor-pointer bg-[#1a1a2e] border-[#2a2a3e] {border_color} {slide_class}",
            style: "{transform_style}",
            onmousedown: move |e: MouseEvent| {
                drag_start_x.set(Some(e.client_coordinates().x));
            },
            onmousemove: move |e: MouseEvent| {
                if let Some(start) = *drag_start_x.read() {
                    let current = e.client_coordinates().x;
                    let diff = current - start;
                    if diff < 0.0 {
                        drag_offset.set(diff);
                    }
                }
            },
            onmouseup: move |_| {
                let off = *drag_offset.read();
                if off < -80.0 {
                    toast_svc.dismiss(id);
                    spawn(async move {
                        #[cfg(feature = "web")]
                        gloo_timers::future::sleep(std::time::Duration::from_millis(300)).await;
                        #[cfg(feature = "server")]
                        tokio::time::sleep(std::time::Duration::from_millis(300)).await;
                        toast_svc.remove(id);
                    });
                } else {
                    drag_offset.set(0.0);
                }
                drag_start_x.set(None);
            },
            onclick: move |_| {
                #[cfg(not(feature = "server"))]
                if let Some(ref url) = link {
                    let _ = web_sys::window().and_then(|w| w.open_with_url(url).ok());
                }
            },

            span { class: "flex-1 text-sm text-[#e0e0e0] select-none", "{toast.message}" }
            if has_link {
                span { class: "text-xs text-blue-400", "↗" }
            }
        }
    }
}
