use bdk::prelude::*;

#[derive(
    Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "snake_case", tag = "answer_type")]
pub enum Attribute {
    Age(Age),
    Gender(Gender),
}

#[derive(
    Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum Age {
    Specific(u8),
    Range {
        inclusive_min: u8,
        inclusive_max: u8,
    },
}

#[derive(
    Debug, Clone, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum Gender {
    Male = 1,
    Female = 2,
}

#[derive(
    Debug,
    Clone,
    Copy,
    Eq,
    PartialEq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum AgeBand {
    U17,
    A18_29,
    A30_39,
    A40_49,
    A50_59,
    A60_69,
    A70P,
}

impl AgeBand {
    pub fn label(self) -> &'static str {
        match self {
            AgeBand::U17 => "0-17",
            AgeBand::A18_29 => "18-29",
            AgeBand::A30_39 => "30-39",
            AgeBand::A40_49 => "40-49",
            AgeBand::A50_59 => "50-59",
            AgeBand::A60_69 => "60-69",
            AgeBand::A70P => "70-",
        }
    }
}

pub fn age_to_band(age: &Age) -> AgeBand {
    let pick = |n: u8| match n {
        0..=17 => AgeBand::U17,
        18..=29 => AgeBand::A18_29,
        30..=39 => AgeBand::A30_39,
        40..=49 => AgeBand::A40_49,
        50..=59 => AgeBand::A50_59,
        60..=69 => AgeBand::A60_69,
        _ => AgeBand::A70P,
    };
    match *age {
        Age::Specific(n) => pick(n),
        Age::Range {
            inclusive_min,
            inclusive_max,
        } => {
            let mid = (inclusive_min as u16 + inclusive_max as u16) / 2;
            pick(mid as u8)
        }
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct RespondentAttr {
    pub gender: Option<Gender>,
    pub age: Option<Age>,
    pub school: Option<String>,
}

impl RespondentAttr {
    pub fn from_attributes(attrs: &[Attribute]) -> Self {
        let mut r = RespondentAttr::default();
        for a in attrs {
            match a {
                Attribute::Gender(g) => r.gender = Some(g.clone()),
                Attribute::Age(a) => r.age = Some(a.clone()),
            }
        }
        r
    }
    pub fn is_empty(&self) -> bool {
        self.gender.is_none() && self.age.is_none() && self.school.is_none()
    }
}
