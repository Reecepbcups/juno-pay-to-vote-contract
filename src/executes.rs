use cosmwasm_std::{Deps, MessageInfo, Response};

use crate::ContractError;
// use crate::msg::{SomeMsg};
// use cosmwasm_std::{Deps, Order, StdResult, Uint128};

use crate::state::{CONFIG, FUNDS};

use crate::coin_helpers::{coin_to_string, convert_vec_coins_to_string};

pub fn execute_add_funds(deps: Deps, info: MessageInfo) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let funds = FUNDS.load(deps.storage)?;

    Ok(Response::new()
        .add_attribute("action", "add_funds")
        .add_attribute("admin", config.admin)
        .add_attribute("added_funds", convert_vec_coins_to_string(info.funds))
        .add_attribute("all_balance", coin_to_string(funds.all_balance)))
}

// pub fn execute_claim_funds(deps: Deps) -> Result<Response, ContractError> {
//     let config = CONFIG.load(deps.storage)?;
//     let funds = FUNDS.load(deps.storage)?;

//     Ok(Response::new().add_attribute("action", "claim_funds"))
// }
