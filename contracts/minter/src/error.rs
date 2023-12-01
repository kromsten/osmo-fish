use cosmwasm_std::StdError;
use cw_utils::PaymentError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Invalid Mint Data")]
    InvalidMintData {},

    #[error("Invalid Amount: Expected {0}, Got {1}")]
    InvalidAmount(u128, u128),

    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Payment(#[from] PaymentError),
}
