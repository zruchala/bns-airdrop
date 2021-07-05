use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, StdResult, Storage};
use cosmwasm_storage::{singleton, singleton_read, ReadonlyBucket, Bucket};

static KEY_CONFIG: &[u8] = b"config";
static LATEST_STAGE: &[u8] = b"latest_stage";
static MERKLE_ROOT: &[u8] = b"merkle_root";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: CanonicalAddr
}

pub fn store_config<S: Storage>(storage: &mut S, config: &Config) -> StdResult<()> {
    singleton(storage, KEY_CONFIG).save(config)
}

pub fn read_config<S: Storage>(storage: &S) -> StdResult<Config> {
    singleton_read(storage, KEY_CONFIG).load()
}

pub fn store_latest_stage<S: Storage>(storage: &mut S, stage: u8) -> StdResult<()> {
    singleton(storage, LATEST_STAGE).save(&stage)
}

pub fn read_latest_stage<S: Storage>(storage: &S) -> StdResult<u8> {
    singleton_read(storage, LATEST_STAGE).load()
}

pub fn store_merkle_root<S: Storage>(
    storage: &mut S,
    stage: u8,
    merkle_root: String) -> StdResult<()> {
    let mut merkle_root_bucket: Bucket<S, String> = Bucket::new(MERKLE_ROOT, storage);
    merkle_root_bucket.save(&[stage], &merkle_root)
}

pub fn read_merkle_root<S: Storage>(
    storage: &S, 
    stage: u8) -> StdResult<String> {
    let claim_index_bucket: ReadonlyBucket<S, String> =
        ReadonlyBucket::new(MERKLE_ROOT, storage);
    claim_index_bucket.load(&[stage])
}
