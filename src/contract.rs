#[cfg(not(feature = "library"))]
// COSMWASM
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, StdError};
// use protobuf::{SpecialFields, Message};

// ERRORS & MESSAGES
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

// CUSTOM CRATES
use crate::queries;
// execute

include!("protos/mod.rs");
// include!("/protos/mod.rs");
// use CosmosGovV1beta1Query::{QueryVoteResponse, Vote, VoteOption, QueryVoteRequest, WeightedVoteOption};

// STATE
use crate::state::{Config, CONFIG};

use cw2::set_contract_version;
const CONTRACT_NAME: &str = "crates.io:juno-paid-to-vote";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // admin if set, if not, the sender = contract admin
    let admin = deps
        .api
        .addr_validate(&msg.contract_admin.unwrap_or(info.sender.to_string()))?;

    let config = Config {
        admin: admin.clone(),
        version: CONTRACT_VERSION.to_string(),
        name: CONTRACT_NAME.to_string(),
        allowed_denom: msg.denom,
    };

    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("admin", admin)
        .add_attribute("version", CONTRACT_VERSION)
        .add_attribute("name", CONTRACT_NAME))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::AddFunds {} => Ok(Response::new().add_attribute("action", "add_funds")),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig{} => to_binary(&queries::query_config(deps)?),
        QueryMsg::QueryVoteStatus { proposal, address } => {
            let bin = queries::query_vote_status(deps, proposal, address);
            if bin.is_ok() {
                to_binary(&true)            
            } else {
                return Err(StdError::generic_err("Error querying vote status"));
            }
        }, 
    }
}

#[entry_point]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    // // https://docs.cosmwasm.com/docs/1.0/smart-contracts/migration/
    let ver = cw2::get_contract_version(deps.storage)?;
    // ensure we are migrating from an allowed contract
    if ver.contract != CONTRACT_NAME {
        return Err(StdError::generic_err("Can only upgrade from same type").into());
    }
    // note: better to do proper semver compare, but string compare *usually* works
    if ver.version >= (*CONTRACT_VERSION).to_string() {
        return Err(StdError::generic_err("Cannot upgrade from a newer version").into());
    }

    // set the new version
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // update the version field in the ContractInfo
    // let mut config = CONFIG.load(deps.storage)?;
    // config.version = CONTRACT_VERSION.to_string();
    // CONTRACT_INFO.save(deps.storage, &config)?;

    Ok(
        Response::default().add_attribute("action", "migration"), // .add_attribute("version", CONTRACT_VERSION)
                                                                  // .add_attribute("contract", CONTRACT_NAME)
    )
}
