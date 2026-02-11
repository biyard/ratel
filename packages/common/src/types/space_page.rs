use crate::*;

#[derive(Debug, Clone, Copy, Translate, PartialEq)]
pub enum SpacePage {
    #[translate(ko = "대시보드")]
    Dashboard,
    #[translate(ko = "개요")]
    Overview,
    #[translate(ko = "미션")]
    Actions,
    #[translate(ko = "앱")]
    Apps,
    #[translate(ko = "보고서")]
    Report,
}
