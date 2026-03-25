use dioxus_translate::{Language, Translate};

#[derive(Debug, Translate, PartialEq, Eq)]
pub enum ProjectArea {
    #[translate(ko = "경제")]
    Economy,
    #[translate(ko = "사회")]
    Society,
    #[translate(ko = "기술")]
    Technology,
    #[translate(ko = "구조체")]
    Struct { a: String, b: i32 },
    #[translate(ko = "튜플")]
    Tuple(String, i32),
}

// Inner error type with per-variant translations
#[derive(Debug, Translate, PartialEq, Eq)]
pub enum InnerError {
    #[translate(en = "Not found", ko = "찾을 수 없습니다")]
    NotFound,
    #[translate(en = "Permission denied", ko = "권한이 없습니다")]
    PermissionDenied,
}

// Outer error type using #[translate(from)] to delegate to InnerError
#[derive(Debug, Translate)]
pub enum OuterError {
    #[translate(en = "Unknown error", ko = "알 수 없는 오류")]
    Unknown(String),
    #[translate(from)]
    Inner(InnerError),
}

#[test]
fn test_translation() {
    assert_eq!(ProjectArea::Economy.translate(&Language::En), "Economy");
    assert_eq!(ProjectArea::Economy.translate(&Language::Ko), "경제");

    assert_eq!(ProjectArea::Society.translate(&Language::En), "Society");
    assert_eq!(ProjectArea::Society.translate(&Language::Ko), "사회");

    assert_eq!(
        ProjectArea::Technology.translate(&Language::En),
        "Technology"
    );
    assert_eq!(ProjectArea::Technology.translate(&Language::Ko), "기술");
    assert_eq!(
        ProjectArea::Struct {
            a: "abg".to_string(),
            b: 3
        }
        .translate(&Language::Ko),
        "구조체"
    );
    assert_eq!(
        ProjectArea::Struct {
            a: "abg".to_string(),
            b: 3
        }
        .translate(&Language::En),
        "Struct"
    );
    assert_eq!(
        ProjectArea::Tuple("abg".to_string(), 3).translate(&Language::Ko),
        "튜플"
    );
    assert_eq!(
        ProjectArea::Tuple("abg".to_string(), 3).translate(&Language::En),
        "Tuple"
    );
}

#[test]
fn test_variants() {
    assert_eq!(
        ProjectArea::VARIANTS,
        &[
            ProjectArea::Economy,
            ProjectArea::Society,
            ProjectArea::Technology,
        ],
    );
}

#[test]
fn test_fn_variants() {
    println!("{:?}", ProjectArea::variants(&Language::Ko));
    assert_eq!(
        ProjectArea::variants(&Language::En),
        vec!["Economy", "Society", "Technology"],
    );
    assert_eq!(
        ProjectArea::variants(&Language::Ko),
        vec!["경제", "사회", "기술"],
    );
}

#[test]
fn test_translate_from_delegates_to_inner() {
    // #[translate(from)] should delegate to InnerError's translate()
    let err = OuterError::Inner(InnerError::NotFound);
    assert_eq!(err.translate(&Language::En), "Not found");
    assert_eq!(err.translate(&Language::Ko), "찾을 수 없습니다");

    let err = OuterError::Inner(InnerError::PermissionDenied);
    assert_eq!(err.translate(&Language::En), "Permission denied");
    assert_eq!(err.translate(&Language::Ko), "권한이 없습니다");

    // Non-from variants should use their own translate attributes
    let err = OuterError::Unknown("test".to_string());
    assert_eq!(err.translate(&Language::En), "Unknown error");
    assert_eq!(err.translate(&Language::Ko), "알 수 없는 오류");
}
