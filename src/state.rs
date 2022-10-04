use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin: Addr,
    pub name: String,
    pub version: String,
    pub allowed_denom: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Funds {
    // TODO: make a map of denom name -> coin?
    pub all_balance: Coin,
}

pub const CONFIG: Item<Config> = Item::new("config");

pub const FUNDS: Item<Funds> = Item::new("funds");
