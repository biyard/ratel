use dioxus_translate::{Language, Translate};
use std::str::FromStr;

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
fn test_display() {
    assert_eq!(format!("{}", ProjectArea::Economy), "economy");
    assert_eq!(format!("{}", ProjectArea::Society), "society");
    assert_eq!(format!("{}", ProjectArea::Technology), "technology");
    assert_eq!(
        format!(
            "{}",
            ProjectArea::Struct {
                a: "abg".to_string(),
                b: 3
            }
        ),
        "struct"
    );
    assert_eq!(
        format!("{}", ProjectArea::Tuple("abg".to_string(), 3)),
        "tuple"
    );
}

#[test]
fn test_from_str() {
    // Test English names
    assert_eq!(ProjectArea::from_str("Economy"), Ok(ProjectArea::Economy));
    assert_eq!(ProjectArea::from_str("Society"), Ok(ProjectArea::Society));
    assert_eq!(
        ProjectArea::from_str("Technology"),
        Ok(ProjectArea::Technology)
    );

    // Test Korean translations
    assert_eq!(ProjectArea::from_str("경제"), Ok(ProjectArea::Economy));
    assert_eq!(ProjectArea::from_str("사회"), Ok(ProjectArea::Society));
    assert_eq!(ProjectArea::from_str("기술"), Ok(ProjectArea::Technology));

    // Test lowercase names
    assert_eq!(ProjectArea::from_str("economy"), Ok(ProjectArea::Economy));
    assert_eq!(ProjectArea::from_str("society"), Ok(ProjectArea::Society));
    assert_eq!(
        ProjectArea::from_str("technology"),
        Ok(ProjectArea::Technology)
    );
    let res = ProjectArea::from_str("구조체");
    assert!(res.is_ok());
    assert_eq!(
        res.unwrap(),
        ProjectArea::Struct {
            a: "".to_string(),
            b: 0
        }
    );
    let res = ProjectArea::from_str("튜플");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), ProjectArea::Tuple("".to_string(), 0));

    // Test invalid input
    assert!(ProjectArea::from_str("invalid_field").is_err());
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
