use cosmwasm_std::{Deps, StdResult, Binary, QueryRequest, StdError};
use protobuf::{Message};

use crate::contract::CosmosGovV1beta1Query::{QueryVoteRequest, QueryVoteResponse};
// use crate::msg::{SomeMsg};
use crate::msg::ConfigResponse;
// use cosmwasm_std::{Deps, Order, StdResult, Uint128};

use crate::state::CONFIG;

pub fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {
        admin: config.admin.to_string(),
        version: config.version,
        name: config.name,
    })
}

pub fn query_vote_status(deps: Deps, proposal: u64, address: String) -> StdResult<bool> {
    // // make a Stargate query
    // let q = QueryRequest::Stargate {
    //     // this is the path to the query
    //     // /cosmos/gov/v1beta1/proposals/{proposal_id}
    //     //
    //     path: format!("gov/proposals/{}", proposal),
    //     data: None,
    // }

    // let mut vote_query = QueryVoteResponse::new();
    // vote_query.vote = Some(protos::cosmos::gov::v1beta1::Vote::new());


    // let final_msg: CosmosMsg = CosmosMsg::Stargate {
    //     type_url: "/cosmos.distribution.v1beta1.MsgWithdrawDelegatorReward".to_string(),
    //     value: Binary::from(withdraw_msg_bytes),
    // };

    // validate address is a valid address
    let addr = deps.api.addr_validate(&address)?;

    let mut vote_query = QueryVoteRequest::new(); // new() also works
    vote_query.proposal_id = proposal;
    vote_query.voter = addr.to_string();
    // vote_query.special_fields = SpecialFields::default(); // we don't need any

    // let query_votes_bytes: Vec<u8> = vote_query.write_to_bytes().unwrap();
    let query_votes_bytes: Vec<u8> = vote_query.write_to_bytes().unwrap();

    // this is an execute you dummy.. useful for the future tho :D
    // let query_user: CosmosMsg = CosmosMsg::Stargate { 
    //     type_url: "/cosmos.gov.v1beta1.QueryVoteRequest".to_string(),
    //     value: Binary::from(query_votes_bytes),
    // };
    
    // let query_vote_resp = ;

    // let query_user_vote: QueryRequest<_> = ;    

    let res_bin: Binary = deps.querier.query(&QueryRequest::Stargate { 
        path: "/cosmos.gov.v1beta1.QueryVoteRequest".to_string(),
        data: Binary::from(query_votes_bytes),
    })?;  

    // make query with query_user_vote as type QueryVoteResponse response
    // let query_vote_resp: QueryResponse = deps.querier.query(&query_user_vote)?;

    // pub mod staking {
    //     include!(concat!(env!("OUT_DIR"), "/cosmos.staking.v1beta1.rs"));
    // }

    // convert binary res_bin to &[u8]
    let res_bin_slice: &[u8] = res_bin.as_slice();

    // convert res_bin to a QueryVoteResponse
    let query_vote_resp: Result<QueryVoteResponse, protobuf::Error> = QueryVoteResponse::parse_from_bytes(res_bin_slice);

    // if some, return the vote status
    if let Ok(_query_vote_resp) = query_vote_resp {
        return Ok(true); // TODO: check if they actually voted.
    } else {
        return Err(StdError::generic_err("Error parsing vote response"));
    }
    
    
    // convert query_vote_resp to QueryVoteResponse
    // let resp: QueryVoteResponse;

    // let resp: QueryVoteResponse = query_vote_resp.into();

    // Ok(
    //     Response::new()
    //     .add_attribute("action", "query_vote_status")
    //     .add_attribute("address", address)
    //     .add_attribute("proposal", proposal.to_string())        
    // )

    // Ok(QueryVoteResponse {
    //     vote: resp.vote,
    //     special_fields: resp.special_fields,         
    // })
    
    // Ok(Response::new()
    //     .add_attribute("contract", "query-voter")
    //     .add_message(query_user_vote)
    // )

    // hmm, how does this work
    // Ok(query_vote_resp)
}