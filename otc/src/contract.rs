#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::state::{Config, Deal, Status, CONFIG, DEALS, LATEST_DEAL_ID};

use cw2::set_contract_version;

use cosmwasm_std::{BankMsg, Coin, DepsMut, Env, MessageInfo, Response, Uint128};

const CONTRACT_NAME: &str = "crates.io:astroport-otc";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    if msg.default_timeout == Some(0) {
        return Err(ContractError::TimeoutCannotBeZero {});
    }

    let default_timeout = msg.default_timeout.unwrap_or(3600);
    let config = Config { default_timeout };
    CONFIG.save(deps.storage, &config)?;
    LATEST_DEAL_ID.save(deps.storage, &0)?;

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("default_timeout", default_timeout.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateDeal {
            denom_in,
            amount_in,
            denom_out,
            amount_out,
            recipient,
            timeout,
        } => try_create_deal(
            deps, env, info, denom_in, amount_in, denom_out, amount_out, recipient, timeout,
        ),
        ExecuteMsg::AcceptDeal {
            deal_id,
            denom_out,
            amount_out,
        } => try_accept_deal(deps, env, info, deal_id, denom_out, amount_out),
        ExecuteMsg::CancelDeal { deal_id } => try_cancel_deal(deps, env, info, deal_id),
        ExecuteMsg::Withdraw { deal_id } => try_withdraw(deps, env, info, deal_id),
    }
}

fn try_create_deal(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    denom_in: String,
    amount_in: Uint128,
    denom_out: String,
    amount_out: Uint128,
    recipient: Option<String>,
    timeout: Option<u64>,
) -> Result<Response, ContractError> {
    if info.funds.len() != 1 {
        return Err(ContractError::InvalidFundsLength {});
    }

    if info.funds[0].denom != denom_in {
        return Err(ContractError::InvalidFundsDenom {});
    }

    if info.funds[0].amount != amount_in {
        return Err(ContractError::InvalidFundsAmount {});
    }

    if timeout == Some(0) {
        return Err(ContractError::TimeoutCannotBeZero {});
    }

    let config = CONFIG.load(deps.storage)?;
    let deal_id = LATEST_DEAL_ID.load(deps.storage)?;
    let valid_recipient = recipient
        .as_ref()
        .map(|addr| deps.api.addr_validate(addr))
        .transpose()?;

    let deal = Deal {
        creator: info.sender,
        recipient: valid_recipient,
        denom_in: denom_in.clone(),
        amount_in,
        denom_out: denom_out.clone(),
        amount_out,
        status: Status::Open,
        timeout: env.block.time.seconds() + timeout.unwrap_or(config.default_timeout),
    };
    DEALS.save(deps.storage, deal_id, &deal)?;

    LATEST_DEAL_ID.save(deps.storage, &(deal_id + 1))?;

    Ok(Response::new()
        .add_attribute("action", "create_deal")
        .add_attribute("deal_id", deal_id.to_string())
        .add_attribute("denom_in", denom_in)
        .add_attribute("amount_in", amount_in.to_string())
        .add_attribute("denom_out", denom_out)
        .add_attribute("amount_out", amount_out.to_string()))
}

fn try_accept_deal(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    deal_id: u64,
    denom_out: String,
    amount_out: Uint128,
) -> Result<Response, ContractError> {
    if !DEALS.has(deps.storage, deal_id) {
        return Err(ContractError::DealNotFound {});
    }

    let mut deal = DEALS.load(deps.storage, deal_id)?;
    if deal.status != Status::Open {
        return Err(ContractError::DealNotOpen {});
    }

    if deal.timeout < env.block.time.seconds() {
        return Err(ContractError::DealExpired {});
    }

    if deal.denom_out != denom_out {
        return Err(ContractError::InvalidFundsDenom {});
    }

    if deal.amount_out != amount_out {
        return Err(ContractError::InvalidFundsAmount {});
    }

    if deal.recipient.is_some() && deal.recipient.clone().unwrap() != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    deal.status = Status::Executed;
    DEALS.save(deps.storage, deal_id, &deal)?;

    let transfer_msg_receiver = BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: vec![Coin {
            denom: denom_out,
            amount: amount_out,
        }],
    };

    let transfer_msg_creator = BankMsg::Send {
        to_address: deal.creator.to_string(),
        amount: info.funds,
    };

    Ok(Response::new()
        .add_attribute("action", "accept_deal")
        .add_attribute("deal_id", deal_id.to_string())
        .add_message(transfer_msg_receiver)
        .add_message(transfer_msg_creator))
}

/// Cancels a deal that has not expired and has not been executed, and returns the funds to the creator
fn try_cancel_deal(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    deal_id: u64,
) -> Result<Response, ContractError> {
    if !DEALS.has(deps.storage, deal_id) {
        return Err(ContractError::DealNotFound {});
    }

    let mut deal = DEALS.load(deps.storage, deal_id)?;
    if deal.status != Status::Open {
        return Err(ContractError::DealNotOpen {});
    }

    if deal.creator != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    if deal.timeout < env.block.time.seconds() {
        return Err(ContractError::DealExpired {});
    }

    deal.status = Status::Cancelled;
    DEALS.save(deps.storage, deal_id, &deal)?;

    let transfer_msg_creator = BankMsg::Send {
        to_address: deal.creator.to_string(),
        amount: info.funds,
    };

    Ok(Response::new()
        .add_attribute("action", "cancel_deal")
        .add_attribute("deal_id", deal_id.to_string())
        .add_message(transfer_msg_creator))
}

/// Sends funds to creator from a deal that has not been executed but has expired
fn try_withdraw(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    deal_id: u64,
) -> Result<Response, ContractError> {
    if !DEALS.has(deps.storage, deal_id) {
        return Err(ContractError::DealNotFound {});
    }

    let deal = DEALS.load(deps.storage, deal_id)?;
    // TODO: figure out how to update expired deals
    if deal.status != Status::Open || deal.status != Status::Expired {
        return Err(ContractError::DealNotExpired {});
    }

    if deal.timeout >= env.block.time.seconds() {
        return Err(ContractError::DealNotExpired {});
    }

    if deal.creator != info.sender {
        return Err(ContractError::InvalidFundsAmount {});
    }

    let transfer_msg = BankMsg::Send {
        to_address: deal.creator.to_string(),
        amount: info.funds,
    };

    Ok(Response::new()
        .add_attribute("action", "withdraw")
        .add_attribute("deal_id", deal_id.to_string())
        .add_message(transfer_msg))
}
