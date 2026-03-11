use super::*;

#[component]
pub fn UploadPage(current_section: Signal<QuizCreatorSection>) -> Element {
    let tr: QuizCreatorTranslate = use_translate();

    rsx! {
        div { class: "flex w-full max-w-[1024px] flex-col gap-6 rounded-[12px] border border-[#404040] bg-[#262626] p-6",
            div { class: "flex flex-col gap-1",
                h3 { class: "text-[24px]/[28px] font-bold tracking-[-0.24px] text-white",
                    {tr.upload_title}
                }
                p { class: "text-[15px]/[22px] font-medium text-[#D4D4D4]",
                    {tr.upload_description}
                }
            }
            div { class: "flex min-h-[240px] w-full items-center justify-center rounded-[12px] border border-dashed border-[#525252] bg-[#101010] text-[#8C8C8C]",
                {tr.upload_placeholder}
            }
            div { class: "flex w-full justify-end gap-3",
                Button {
                    style: ButtonStyle::Outline,
                    shape: ButtonShape::Square,
                    class: "min-w-[110px]",
                    onclick: move |_| current_section.set(QuizCreatorSection::Overview),
                    {tr.btn_back}
                }
                Button {
                    style: ButtonStyle::Primary,
                    shape: ButtonShape::Square,
                    class: "min-w-[110px]",
                    onclick: move |_| current_section.set(QuizCreatorSection::Quiz),
                    "{tr.btn_next} ->"
                }
            }
        }
    }
}
