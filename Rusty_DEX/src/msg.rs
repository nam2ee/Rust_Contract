use cosmwasm_std::{Binary, Decimal, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub token1_denom: String,
    pub token2_denom: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ExecuteMsg {
    AddLiquidity { amount1: Uint128, amount2: Uint128 },
    RemoveLiquidity { shares: Uint128 },
    Swap { input_token: String, input_amount: Uint128 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryMsg {
    GetPool {},
    GetPrice { token: String, amount: Uint128 },
}
