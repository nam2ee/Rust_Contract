use cosmwasm_std::{Addr, Uint128};
use cosmwasm_storage::{singleton, singleton_read, Singleton};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub static CONFIG_KEY: &[u8] = b"config";
pub static POOL_KEY: &[u8] = b"pool";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub token1_denom: String,
    pub token2_denom: String,
    pub owner: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Pool {
    pub token1_reserve: Uint128,
    pub token2_reserve: Uint128,
    pub total_shares: Uint128,
}

pub fn store_config(storage: &mut dyn cosmwasm_std::Storage, config: &Config) -> cosmwasm_std::StdResult<()> {
    singleton(storage, CONFIG_KEY).save(config)
}

pub fn load_config(storage: &dyn cosmwasm_std::Storage) -> cosmwasm_std::StdResult<Config> {
    singleton_read(storage, CONFIG_KEY).load()
}

pub fn store_pool(storage: &mut dyn cosmwasm_std::Storage, pool: &Pool) -> cosmwasm_std::StdResult<()> {
    singleton(storage, POOL_KEY).save(pool)
}

pub fn load_pool(storage: &dyn cosmwasm_std::Storage) -> cosmwasm_std::StdResult<Pool> {
    singleton_read(storage, POOL_KEY).load()
}
