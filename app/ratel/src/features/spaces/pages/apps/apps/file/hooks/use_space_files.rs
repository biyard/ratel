use crate::features::spaces::pages::apps::apps::file::controllers::*;
use crate::*;

/// Which pseudo-category the user is currently filtering the file
/// list by. The app lives inside the page as reactive state (not
/// persisted server-side) so it lives on the controller struct.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileTab {
    All,
    Overview,
    Boards,
    Quiz,
}

/// Controller hook for the Space File app (read-only arena view).
///
/// Owns:
/// - `files` — raw `Vec<File>` stored on the space (uploaded assets).
/// - `file_links` — link rows that tag each file with a target
///   (Overview, Board(discussion_id), Quiz(quiz_id)). Used to derive
///   which files show under each tab.
/// - `active_tab` — client-only filter signal.
///
/// No mutation actions — the arena view is read-only. Uploads /
/// deletions stay out of this controller so the page can't accidentally
/// mutate state via a stray hook consumer.
#[derive(Clone, Copy)]
pub struct UseSpaceFiles {
    pub space_id: ReadSignal<SpacePartition>,
    pub files: Loader<Vec<File>>,
    pub file_links: Loader<Vec<FileLinkInfo>>,
    pub active_tab: Signal<FileTab>,
}

#[track_caller]
pub fn use_space_files(
    space_id: ReadSignal<SpacePartition>,
) -> std::result::Result<UseSpaceFiles, RenderError> {
    if let Some(ctx) = try_use_context::<UseSpaceFiles>() {
        return Ok(ctx);
    }

    let files = use_loader(move || async move { get_space_files(space_id()).await })?;
    let file_links = use_loader(move || async move { list_file_links(space_id()).await })?;
    let active_tab = use_signal(|| FileTab::All);

    Ok(use_context_provider(|| UseSpaceFiles {
        space_id,
        files,
        file_links,
        active_tab,
    }))
}
