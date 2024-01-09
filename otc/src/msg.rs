use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;

#[cw_serde]
pub struct InstantiateMsg {
    pub default_timeout: Option<u64>,
}

#[cw_serde]
pub enum ExecuteMsg {
    CreateDeal {
        denom_in: String,
        amount_in: Uint128,
        denom_out: String,
        amount_out: Uint128,
        recipient: Option<String>,
        timeout: Option<u64>,
    },
    AcceptDeal {
        deal_id: u64,
        denom_out: String,
        amount_out: Uint128,
    },
    CancelDeal {
        deal_id: u64,
    },
    Withdraw {
        deal_id: u64,
    },
}

#[cw_serde]
pub enum QueryMsg {}
