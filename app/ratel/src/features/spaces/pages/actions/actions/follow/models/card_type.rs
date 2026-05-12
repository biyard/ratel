use crate::features::spaces::pages::actions::actions::follow::*;
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub enum CardType {
    #[default]
    Full,
    Small,
}
