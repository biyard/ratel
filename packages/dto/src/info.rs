use bdk::prelude::*;

#[derive(Debug, PartialEq, Eq, Translate)]
pub enum Info {
    #[translate(en = "You've successfully liked the election pledge")]
    LikePledge,
}
