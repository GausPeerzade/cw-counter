use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct config {
    pub last_incrimenter: Addr,
    pub counter: u64,
}

pub const CONFIG: Item<config> = Item::new("config");
