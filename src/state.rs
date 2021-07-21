use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, StdResult, Storage};
use cosmwasm_storage::{singleton, singleton_read, ReadonlyBucket, Bucket};

static CONFIG: &[u8] = b"config";
static STAGE_INDEX: &[u8] = b"stage_index";
static MERKLE_ROOT: &[u8] = b"merkle_root";
static CLAIM_INDEX: &[u8] = b"claim_index";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: CanonicalAddr, // contract owner
    pub token: CanonicalAddr  // cw20 token contract address
}

pub fn store_config<S: Storage>(storage: &mut S, config: &Config) -> StdResult<()> {
    singleton(storage, CONFIG).save(config)
}

pub fn read_config<S: Storage>(storage: &S) -> StdResult<Config> {
    singleton_read(storage, CONFIG).load()
}

pub fn store_stage_index<S: Storage>(storage: &mut S, stage: u8) -> StdResult<()> {
    singleton(storage, STAGE_INDEX).save(&stage)
}

pub fn read_stage_index<S: Storage>(storage: &S) -> StdResult<u8> {
    singleton_read(storage, STAGE_INDEX).load()
}

pub fn store_merkle_root<S: Storage>(
    storage: &mut S,
    stage_index: u8,
    merkle_root: String) -> StdResult<()> {
    let mut merkle_root_bucket: Bucket<S, String> = Bucket::new(MERKLE_ROOT, storage);
    merkle_root_bucket.save(&[stage_index], &merkle_root)
}

pub fn read_merkle_root<S: Storage>(
    storage: &S,
    stage_index: u8) -> StdResult<String> {
    let claim_index_bucket: ReadonlyBucket<S, String> =
        ReadonlyBucket::new(MERKLE_ROOT, storage);
    claim_index_bucket.load(&[stage_index])
}

pub fn store_claimed<S: Storage>(
    storage: &mut S,
    address: &CanonicalAddr,
    stage_index: u8,
) -> StdResult<()> {
    let mut claim_index_bucket: Bucket<S, bool> =
        Bucket::multilevel(&[CLAIM_INDEX, address.as_slice()], storage);
    claim_index_bucket.save(&[stage_index], &true)
}

pub fn read_claimed<S: Storage>(
    storage: &S,
    address: &CanonicalAddr,
    stage_index: u8
) -> StdResult<bool> {
    let claim_index_bucket: ReadonlyBucket<S, bool> =
        ReadonlyBucket::multilevel(&[CLAIM_INDEX, address.as_slice()], storage);
    let res = claim_index_bucket.may_load(&[stage_index])?;
    match res {
        Some(v) => Ok(v),
        None => Ok(false),
    }
}
