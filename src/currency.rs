use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum Currency {
    NOK,
    SEK,
    DKK,
    ISK,
    GBP,
}
