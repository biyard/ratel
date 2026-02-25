use common::attribute::{Age, Attribute, Gender};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
                Attribute::Gender(g) => r.gender = Some(*g),
                Attribute::Age(a) => r.age = Some(*a),
            }
        }
        r
    }

    pub fn is_empty(&self) -> bool {
        self.gender.is_none() && self.age.is_none() && self.school.is_none()
    }
}
