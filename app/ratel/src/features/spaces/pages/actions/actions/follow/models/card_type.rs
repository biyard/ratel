use crate::features::spaces::pages::actions::actions::follow::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum CardType {
    #[default]
    Full,
    Small,
}
