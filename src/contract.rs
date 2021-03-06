use cosmwasm_std::{Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, StdError, StdResult, Storage, log, to_binary, CosmosMsg, WasmMsg, Uint128, HumanAddr};
use crate::msg::{InitMsg, HandleMsg, QueryMsg};
use crate::state::{Config, read_config, store_config, store_stage_index, store_merkle_root, read_stage_index, read_merkle_root, read_claimed, store_claimed};

use hex;
use sha3::{Keccak256, Digest};
use std::convert::TryInto;
use cw20::{Cw20HandleMsg};

const LEAF: u8 = 0x00;
const INTERIOR: u8 = 0x01;

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {

    // the address which initiates the contract becomes its owner.
    store_config(
        &mut deps.storage,
        &Config {
            owner: deps.api.canonical_address(&msg.owner)?,
            token: deps.api.canonical_address(&msg.token)?,
        },
    )?;
    store_stage_index(&mut deps.storage, 0)?;

    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::MerkleRoot { node: merkle_root } => {
            register_merkle_root(deps, env, merkle_root)
        }
        HandleMsg::Claim { stage_index, amount, proof} => {
            claim(deps, env, stage_index, amount, proof)
        }
    }
}

pub fn register_merkle_root<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    merkle_root: String,
) -> StdResult<HandleResponse> {

    is_authorized(deps, env)?;
    check_merkle_root(merkle_root.to_string())?;

    let incremented_stage_index = read_stage_index(&deps.storage)? + 1;

    store_merkle_root(&mut deps.storage, incremented_stage_index, merkle_root.to_string())?;
    store_stage_index(&mut deps.storage, incremented_stage_index)?;

    Ok(HandleResponse {
        messages: vec![],
        log: vec![
            log("action", "register_merkle_root"),
            log("merkle_root", merkle_root.to_string()),
            log("stage_index", incremented_stage_index),
        ],
        data: None,
    })
}

fn is_authorized<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>, env: Env) -> StdResult<()> {
    let config: Config = read_config(&deps.storage)?;
    if deps.api.canonical_address(&env.message.sender)? != config.owner {
        return Err(StdError::unauthorized());
    }
    Ok(())
}

fn check_merkle_root(merkle_root: String) -> StdResult<()> {
    let mut buff: [u8; 32] = [0; 32];
    match hex::decode_to_slice(merkle_root, &mut buff) {
        Ok(_) => Ok(()),
        _ => Err(StdError::generic_err("Invalid merkle root hex encoding")),
    }
}

pub fn claim<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    stage_index: u8,
    amount: Uint128,
    proof: Vec<String>
) -> StdResult<HandleResponse> {

    let claiming_address = &deps.api.canonical_address(&env.message.sender)?;
    // check whether the airdrop is not claimed already
    if read_claimed(&deps.storage, claiming_address, stage_index)? {
        return Err(StdError::GenericErr {
            msg: String::from("Already claimed"),
            backtrace: None
        });
    }

    let mut merkle_root: [u8; 32] = [0; 32];
    let root = read_merkle_root(&deps.storage, stage_index)?;
    hex::decode_to_slice(root, &mut merkle_root).unwrap();

    if merkle_root != calculate_root(&env.message.sender, &amount, proof) {
        return Err(StdError::generic_err("The proof presented is invalid"));
    }

    let config: Config = read_config(&deps.storage)?;
    // transfer token to msg sender
    let cosmos_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: deps.api.human_address(&config.token)?,
        send: vec![],
        msg: to_binary(&Cw20HandleMsg::Transfer {
            recipient: env.message.sender.clone(),
            amount
        })?
    });

    // Mark claiming_address to disallow taking the airdrop more than once ...
    store_claimed(&mut deps.storage, claiming_address, stage_index)?;

    Ok(HandleResponse {
        messages: vec![cosmos_msg],
        log: vec![
            log("action", "claim"),
            log("stage_index", stage_index),
            log("address", claiming_address),
            log("amount", amount)
        ],
        data: None
    })
}

fn calculate_root(sender: &HumanAddr, amount: &Uint128, proof: Vec<String>) -> [u8; 32] {
    let leaf = [sender.to_string(), amount.to_string()].join(":");
    let leaf_hash: [u8; 32] = Keccak256::digest(leaf.as_bytes())
        .as_slice().try_into().expect("Inconvertible sender address");

    let mut prefixed_leaf_hash: [u8; 33] = [LEAF; 33];
    prefixed_leaf_hash[1..].copy_from_slice(&leaf_hash);

    let mut outcome: [u8; 32] = Keccak256::digest(&prefixed_leaf_hash)
        .as_slice().try_into().expect("Conversion error");
    for node_hash in proof {
        let proof_hash: [u8; 32] = node_hash.as_bytes().try_into().expect("Conversion error");
        let mut parts: [u8; 3] = [INTERIOR; 3];
        parts[1..].copy_from_slice(&sort_nodes(outcome, proof_hash).concat());
        outcome = Keccak256::digest(&parts)
            .as_slice().try_into().expect("Conversion error");
    }
    outcome
}

fn sort_nodes(left: [u8; 32], right: [u8; 32]) -> [[u8;32]; 2] {
    let mut i = 0;
    while i < 32 {
        if left[i] > right[i] {
            return [right, left];
        } else if left[i] < right[i] {
            return [left, right];
        }
        i += 1;
    }
    [left, right]
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&read_config(&deps.storage)?),
        QueryMsg::StageIndex {} => to_binary(&read_stage_index(&deps.storage)?),
        QueryMsg::MerkleRoot { stage_index } => to_binary(&read_merkle_root(&deps.storage, stage_index)?),
        QueryMsg::IsClaimed { stage_index, address } =>
            to_binary(&read_claimed(&deps.storage, &deps.api.canonical_address(&address)?, stage_index)),
    }

}
