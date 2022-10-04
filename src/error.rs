use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },

    #[error("Invalid Query: {query:?}")]
    InvalidQuery { query: String },

    #[error("Invalid Input needed {needed:?} but received {received:?}")]
    InsufficientFundsSend { needed: String, received: String },
}
