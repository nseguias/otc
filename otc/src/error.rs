use cosmwasm_std::{OverflowError, StdError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("Standard error: {0}")]
    Std(#[from] StdError),

    #[error("Must sent only one coin")]
    InvalidFundsLength {},
    #[error("Timeout cannot be zero")]
    TimeoutCannotBeZero {},
    #[error("Invalid funds denom")]
    InvalidFundsDenom {},
    #[error("Invalid funds amount")]
    InvalidFundsAmount {},
    #[error("Deal not open")]
    DealNotOpen {},
    #[error("{0}")]
    OverflowError(#[from] OverflowError),
    #[error("Deal expired")]
    DealExpired {},
    #[error("Deal not found")]
    DealNotFound {},
    #[error("Unauthorized")]
    Unauthorized {},
    #[error("Deal not expired")]
    DealNotExpired {},
}
