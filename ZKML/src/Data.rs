use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{StdError, StdResult};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Model {
    pub weights: Vec<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InputData {
    pub inputs: Vec<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OutputData {
    pub outputs: Vec<f64>,
}
