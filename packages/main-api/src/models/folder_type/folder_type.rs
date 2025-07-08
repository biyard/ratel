use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FolderType {
    Video,
    Audio,
    MeetingEvents,
    Etc,
}

impl FromStr for FolderType {
    type Err = ();

    fn from_str(key: &str) -> Result<Self, Self::Err> {
        if key.contains("/video/") {
            Ok(FolderType::Video)
        } else if key.contains("/audio/") {
            Ok(FolderType::Audio)
        } else if key.contains("/meeting-events/") {
            Ok(FolderType::MeetingEvents)
        } else {
            Ok(FolderType::Etc)
        }
    }
}

impl fmt::Display for FolderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            FolderType::Video => "video",
            FolderType::Audio => "audio",
            FolderType::MeetingEvents => "meeting-events",
            FolderType::Etc => "etc",
        };
        write!(f, "{}", s)
    }
}
