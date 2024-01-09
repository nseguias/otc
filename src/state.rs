use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub enum Status {
    Open,
    Executed,
    Expired,
    Cancelled,
}

#[cw_serde]
pub struct Deal {
    pub creator: Addr,
    pub recipient: Option<Addr>,
    pub denom_in: String,
    pub amount_in: Uint128,
    pub denom_out: String,
    pub amount_out: Uint128,
    pub status: Status,
    pub timeout: u64,
}

#[cw_serde]
pub struct Config {
    pub default_timeout: u64,
}

pub const CONFIG: Item<Config> = Item::new("config");

// increments on every deal creation
pub const LATEST_DEAL_ID: Item<u64> = Item::new("latest_deal_id");

// maps deal_id => deal
pub const DEALS: Map<u64, Deal> = Map::new("deals");
