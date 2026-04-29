use super::*;
use crate::features::spaces::space_common::hooks::use_space;

const DEFAULT_SPACE_LOGO: &str = "https://metadata.ratel.foundation/logos/logo-symbol.png";

fn is_image_ext(ext: &FileExtension) -> bool {
    matches!(ext, FileExtension::JPG | FileExtension::PNG)
}

fn is_video_ext(ext: &FileExtension) -> bool {
    matches!(
        ext,
        FileExtension::MP4 | FileExtension::MOV | FileExtension::MKV
    )
}

fn is_doc_ext(ext: &FileExtension) -> bool {
    matches!(
        ext,
        FileExtension::PDF | FileExtension::WORD | FileExtension::PPTX | FileExtension::EXCEL
    )
}

fn ext_label(ext: &FileExtension) -> &'static str {
    match ext {
        FileExtension::JPG => "JPG",
        FileExtension::PNG => "PNG",
        FileExtension::PDF => "PDF",
        FileExtension::ZIP => "ZIP",
        FileExtension::WORD => "DOC",
        FileExtension::PPTX => "PPT",
        FileExtension::EXCEL => "XLS",
        FileExtension::MP4 => "MP4",
        FileExtension::MOV => "MOV",
        FileExtension::MKV => "MKV",
    }
}

fn icon_modifier(ext: &FileExtension) -> &'static str {
    if is_image_ext(ext) {
        "sfa-file-card__icon sfa-file-card__icon--img"
    } else if is_video_ext(ext) {
        "sfa-file-card__icon sfa-file-card__icon--vid"
    } else if matches!(ext, FileExtension::ZIP) {
        "sfa-file-card__icon sfa-file-card__icon--zip"
    } else if is_doc_ext(ext) {
        "sfa-file-card__icon sfa-file-card__icon--doc"
    } else {
        "sfa-file-card__icon sfa-file-card__icon--doc"
    }
}

/// Collect files whose URL is referenced by a link matching `predicate`.
/// Used per-tab to derive the displayed subset from the base `files`
/// list and the space's `file_links` rows.
fn filter_by_target(
    all_files: &[File],
    links: &[FileLinkInfo],
    predicate: impl Fn(&FileLinkTarget) -> bool,
) -> Vec<File> {
    let linked_urls: Vec<String> = links
        .iter()
        .filter(|link| predicate(&link.link_target))
        .map(|link| link.file_url.clone())
        .collect();
    all_files
        .iter()
        .filter(|f| {
            f.url
                .as_ref()
                .is_some_and(|url| linked_urls.contains(url))
        })
        .cloned()
        .collect()
}

/// Look up the first link for a file so we can render a Tag chip
/// (Overview / Board / Quiz) next to the file name. Returns `None`
/// when the file isn't linked anywhere.
fn link_for<'a>(file: &File, links: &'a [FileLinkInfo]) -> Option<&'a FileLinkInfo> {
    let url = file.url.as_ref()?;
    links.iter().find(|link| &link.file_url == url)
}

fn tag_class(target: &FileLinkTarget) -> Option<&'static str> {
    match target {
        FileLinkTarget::Overview => {
            Some("sfa-file-card__tag sfa-file-card__tag--overview")
        }
        FileLinkTarget::Board(_) => Some("sfa-file-card__tag sfa-file-card__tag--board"),
        FileLinkTarget::Quiz(_) => Some("sfa-file-card__tag sfa-file-card__tag--quiz"),
        FileLinkTarget::Files => None,
    }
}

fn tag_label(target: &FileLinkTarget) -> &'static str {
    match target {
        FileLinkTarget::Overview => "Overview",
        FileLinkTarget::Board(_) => "Board",
        FileLinkTarget::Quiz(_) => "Quiz",
        FileLinkTarget::Files => "",
    }
}

#[component]
pub fn SpaceFileAppPage(space_id: ReadSignal<SpacePartition>) -> Element {
    let tr: SpaceFileTranslate = use_translate();
    let space = use_space();
    let nav = use_navigator();

    // Instantiate the read-only controller at the page root.
    let UseSpaceFiles {
        files,
        file_links,
        mut active_tab,
        ..
    } = use_space_files(space_id)?;

    let all_files = files();
    let links = file_links();
    let current_tab = active_tab();

    let displayed_files: Vec<File> = match current_tab {
        FileTab::All => all_files.clone(),
        FileTab::Overview => filter_by_target(&all_files, &links, |t| {
            matches!(t, FileLinkTarget::Overview)
        }),
        FileTab::Boards => filter_by_target(&all_files, &links, |t| {
            matches!(t, FileLinkTarget::Board(_))
        }),
        FileTab::Quiz => filter_by_target(&all_files, &links, |t| {
            matches!(t, FileLinkTarget::Quiz(_))
        }),
    };

    let image_files: Vec<File> = displayed_files
        .iter()
        .filter(|f| is_image_ext(&f.ext))
        .cloned()
        .collect();
    let video_files: Vec<File> = displayed_files
        .iter()
        .filter(|f| is_video_ext(&f.ext))
        .cloned()
        .collect();

    let space_data = space();
    let space_logo = if space_data.logo.is_empty() {
        DEFAULT_SPACE_LOGO.to_string()
    } else {
        space_data.logo.clone()
    };
    let space_title = space_data.title.clone();
    let total_count = all_files.len();
    let displayed_count = displayed_files.len();

    rsx! {

        div { class: "space-files-arena",
            // ── Arena topbar ────────────────────────────
            header { class: "sfa-topbar", role: "banner",
                div { class: "sfa-topbar__left",
                    button {
                        r#type: "button",
                        class: "sfa-back-btn",
                        "aria-label": "Back",
                        "data-testid": "topbar-back",
                        onclick: move |_| {
                            // Follow browser history — whichever page
                            // the user was on before landing here.
                            nav.go_back();
                        },
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            "stroke-width": "2",
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            path { d: "M19 12H5" }
                            path { d: "M12 19l-7-7 7-7" }
                        }
                    }
                    img {
                        class: "sfa-topbar__logo",
                        alt: "Space logo",
                        src: "{space_logo}",
                    }
                    nav { class: "sfa-breadcrumb",
                        span { class: "sfa-breadcrumb__item", "{space_title}" }
                        span { class: "sfa-breadcrumb__sep", "›" }
                        span { class: "sfa-breadcrumb__item", "Apps" }
                        span { class: "sfa-breadcrumb__sep", "›" }
                        span { class: "sfa-breadcrumb__item sfa-breadcrumb__current",
                            "File"
                        }
                    }
                    span { class: "sfa-type-badge",
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            "stroke-width": "2",
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            path { d: "M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" }
                            polyline { points: "14 2 14 8 20 8" }
                        }
                        "File"
                    }
                    span { class: "sfa-topbar-title", "File Library" }
                }
            }

            // ── Main body ───────────────────────────────
            main { class: "sfa-body",
                // Tabs + count
                div { class: "sfa-tabs",
                    div { class: "sfa-segmented", "data-testid": "file-tabs",
                        button {
                            r#type: "button",
                            class: "sfa-segmented__btn",
                            "aria-selected": current_tab == FileTab::All,
                            "data-testid": "tab-all",
                            onclick: move |_| active_tab.set(FileTab::All),
                            {tr.tab_all}
                        }
                        button {
                            r#type: "button",
                            class: "sfa-segmented__btn",
                            "aria-selected": current_tab == FileTab::Overview,
                            "data-testid": "tab-overview",
                            onclick: move |_| active_tab.set(FileTab::Overview),
                            {tr.tab_overview}
                        }
                        button {
                            r#type: "button",
                            class: "sfa-segmented__btn",
                            "aria-selected": current_tab == FileTab::Boards,
                            "data-testid": "tab-boards",
                            onclick: move |_| active_tab.set(FileTab::Boards),
                            {tr.tab_boards}
                        }
                        button {
                            r#type: "button",
                            class: "sfa-segmented__btn",
                            "aria-selected": current_tab == FileTab::Quiz,
                            "data-testid": "tab-quiz",
                            onclick: move |_| active_tab.set(FileTab::Quiz),
                            {tr.tab_quiz}
                        }
                    }
                    div { class: "sfa-tabs__meta",
                        span {
                            class: "sfa-tab-count",
                            "data-testid": "file-count",
                            strong { "{displayed_count}" }
                            "files"
                        }
                    }
                }

                // File list
                section { class: "sfa-section", "data-testid": "section-files",
                    div { class: "sfa-section__head",
                        span { class: "sfa-section__label", "Files" }
                        span { class: "sfa-section__hint", "Click a card to open" }
                    }
                    if displayed_files.is_empty() {
                        div { class: "sfa-empty",
                            span { class: "sfa-empty__icon",
                                svg {
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    "stroke-width": "1.8",
                                    "stroke-linecap": "round",
                                    "stroke-linejoin": "round",
                                    path { d: "M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" }
                                    polyline { points: "14 2 14 8 20 8" }
                                }
                            }
                            span { class: "sfa-empty__title", "{tr.no_files}" }
                            span { class: "sfa-empty__sub",
                                "Files shared in this space will appear here"
                            }
                        }
                    } else {
                        div {
                            class: "sfa-file-list",
                            "data-testid": "file-list",
                            for file in displayed_files.iter() {
                                FileRow {
                                    key: "{file.id}",
                                    file: file.clone(),
                                    link: link_for(file, &links).cloned(),
                                }
                            }
                        }
                    }
                }

                // Image previews
                if !image_files.is_empty() {
                    section {
                        class: "sfa-section",
                        "data-testid": "section-image-preview",
                        div { class: "sfa-preview-group__head",
                            span { class: "sfa-preview-group__title", "Image previews" }
                            span { class: "sfa-preview-group__count", "{image_files.len()}" }
                        }
                        div { class: "sfa-preview-grid",
                            for file in image_files.iter() {
                                if let Some(url) = file.url.clone().filter(|u| !u.is_empty()) {
                                    div {
                                        key: "img-{file.id}",
                                        class: "sfa-preview-media",
                                        img { src: "{url}", alt: "{file.name}" }
                                        span { class: "sfa-preview-media__label",
                                            "{ext_label(&file.ext)}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Video previews
                if !video_files.is_empty() {
                    section {
                        class: "sfa-section",
                        "data-testid": "section-video-preview",
                        div { class: "sfa-preview-group__head",
                            span { class: "sfa-preview-group__title", "Video previews" }
                            span { class: "sfa-preview-group__count", "{video_files.len()}" }
                        }
                        div {
                            class: "sfa-preview-grid",
                            style: "grid-template-columns:1fr",
                            for file in video_files.iter() {
                                if let Some(url) = file.url.clone().filter(|u| !u.is_empty()) {
                                    div {
                                        key: "video-{file.id}",
                                        class: "sfa-preview-media",
                                        style: "aspect-ratio:16/7",
                                        video { src: "{url}", controls: true }
                                        span { class: "sfa-preview-media__label",
                                            "{ext_label(&file.ext)}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // ── Sticky footer ──────────────────────────
            footer { class: "sfa-footer",
                span { class: "sfa-footer__left", "{total_count} files total" }
            }
        }
    }
}

#[component]
fn FileRow(file: File, link: Option<FileLinkInfo>) -> Element {
    let url = file.url.clone().unwrap_or_default();
    let can_open = !url.is_empty();
    let size = file.size.clone();
    let name = file.name.clone();
    let ext_text = ext_label(&file.ext);
    let icon_cls = icon_modifier(&file.ext);

    let tag_node: Element = match link.as_ref().and_then(|l| {
        let cls = tag_class(&l.link_target)?;
        Some((cls, tag_label(&l.link_target)))
    }) {
        Some((cls, label)) => rsx! {
            span { class: "{cls}", "{label}" }
        },
        None => rsx! {},
    };

    let row = rsx! {
        div { class: "{icon_cls}", "{ext_text}" }
        div { class: "sfa-file-card__body",
            span { class: "sfa-file-card__name", "{name}" }
            span { class: "sfa-file-card__meta",
                span { "{size}" }
                {tag_node}
            }
        }
    };

    if can_open {
        rsx! {
            a {
                class: "sfa-file-card",
                href: "{url}",
                target: "_blank",
                rel: "noopener noreferrer",
                "data-testid": "file-card",
                {row}
            }
        }
    } else {
        rsx! {
            div { class: "sfa-file-card", "data-testid": "file-card", {row} }
        }
    }
}
