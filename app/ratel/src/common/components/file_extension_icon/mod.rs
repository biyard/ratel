use crate::common::*;

#[component]
pub fn FileExtensionIcon(
    ext: FileExtension,
    #[props(default = 36)] size: u32,
) -> Element {
    let size_attr = icon_size_attr(size);
    let icon_class = "text-card-meta [&>path:first-child]:fill-transparent [&>path:first-child]:stroke-current [&>rect:first-child]:fill-transparent [&>rect:first-child]:stroke-current [&_g>path:first-child]:fill-transparent [&_g>path:first-child]:stroke-current";

    match ext {
        FileExtension::PDF => rsx! {
            icons::files::Pdf { width: size_attr, height: size_attr, class: icon_class }
        },
        FileExtension::JPG => rsx! {
            icons::files::Jpg { width: size_attr, height: size_attr, class: icon_class }
        },
        FileExtension::PNG => rsx! {
            icons::files::Png { width: size_attr, height: size_attr, class: icon_class }
        },
        FileExtension::ZIP => rsx! {
            icons::files::Zip { width: size_attr, height: size_attr, class: icon_class }
        },
        FileExtension::WORD => rsx! {
            icons::files::Docx { width: size_attr, height: size_attr, class: icon_class }
        },
        FileExtension::PPTX => rsx! {
            icons::files::Pptx { width: size_attr, height: size_attr, class: icon_class }
        },
        FileExtension::EXCEL => rsx! {
            icons::files::Xlsx { width: size_attr, height: size_attr, class: icon_class }
        },
        FileExtension::MP4 => rsx! {
            icons::files::Mp4 { width: size_attr, height: size_attr, class: icon_class }
        },
        FileExtension::MOV => rsx! {
            icons::files::Mov { width: size_attr, height: size_attr, class: icon_class }
        },
        FileExtension::MKV => rsx! {
            icons::file::File {
                width: size_attr,
                height: size_attr,
                class: "text-card-meta [&>path]:stroke-current [&>path]:fill-none",
            }
        },
    }
}

fn icon_size_attr(size: u32) -> &'static str {
    match size {
        14 => "14",
        16 => "16",
        20 => "20",
        24 => "24",
        32 => "32",
        36 => "36",
        40 => "40",
        _ => "36",
    }
}
