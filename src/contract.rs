use cosmwasm_std::{Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, StdError, StdResult, Storage, log, to_binary};

use crate::msg::{InitMsg, HandleMsg, QueryMsg};
use crate::state::{Config, read_config, store_config, store_latest_stage, store_merkle_root, read_latest_stage, read_merkle_root};

use hex;

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {

    store_config(
        &mut deps.storage,
        &Config {
            owner: deps.api.canonical_address(&msg.owner)?,
        },
    )?;

    store_latest_stage(&mut deps.storage, 0)?;

    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::RegisterMerkleRoot { merkle_root } => {
            register_merkle_root(deps, env, merkle_root)
        }
    }
}

pub fn register_merkle_root<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    merkle_root: String,
) -> StdResult<HandleResponse> {
    let config: Config = read_config(&deps.storage)?;
    if deps.api.canonical_address(&env.message.sender)? != config.owner {
        return Err(StdError::unauthorized());
    }

    let mut root_buf: [u8; 32] = [0; 32];
    match hex::decode_to_slice(merkle_root.to_string(), &mut root_buf) {
        Ok(()) => {}
        _ => return Err(StdError::generic_err("Invalid hex encoded merkle root")),
    }

    let latest_stage: u8 = read_latest_stage(&deps.storage)?;
    let stage = latest_stage + 1;

    store_merkle_root(&mut deps.storage, stage, merkle_root.to_string())?;
    store_latest_stage(&mut deps.storage, stage)?;

    Ok(HandleResponse {
        messages: vec![],
        log: vec![
            log("action", "register_merkle_root"),
            log("stage", stage),
            log("merkle_root", merkle_root),
        ],
        data: None,
    })
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&read_config(&deps.storage)?),
        QueryMsg::LatestStage {} => to_binary(&read_latest_stage(&deps.storage)?),
        QueryMsg::MerkleRoot { stage } => to_binary(&read_merkle_root(&deps.storage, stage)?),
    }
}
