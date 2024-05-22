use cosmwasm_std::{
    to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, StdError,
    StdResult, Storage,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct HandleMsg {
    pub update_model: Option<Vec<(Vec<u8>, u64)>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct QueryMsg {
    pub get_model: bool,
}

pub fn init<S: Storage, A: Api, Q: Querier>(
    _deps: &mut Extern<S, A, Q>,
    _env: Env,
    _msg: InitMsg,
) -> StdResult<InitResponse> {
    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg { update_model } => try_update_model(deps, update_model),
    }
}

pub fn try_update_model<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    models: Option<Vec<(Vec<u8>, u64)>>,
) -> StdResult<HandleResponse> {
    if let Some(model_data) = models {
        // Perform weighted aggregation of models
        let aggregated_model = aggregate_models(model_data);
        
        // Save the aggregated model
        deps.storage.set(b"model", &aggregated_model);
        Ok(HandleResponse::default())
    } else {
        Err(StdError::generic_err("Model data not provided"))
    }
}

fn aggregate_models(models: Vec<(Vec<u8>, u64)>) -> Vec<u8> {
    // Perform weighted aggregation of models
    let total_weight: u64 = models.iter().map(|(_, weight)| weight).sum();
    let mut aggregated_model = vec![0; models[0].0.len()];
    
    for (model, weight) in models {
        for (i, &value) in model.iter().enumerate() {
            aggregated_model[i] += (value as u64 * weight) / total_weight;
        }
    }
    
    aggregated_model.iter().map(|&x| x as u8).collect()
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg { get_model } => to_binary(&query_model(deps, get_model)?),
    }
}

fn query_model<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    get_model: bool,
) -> StdResult<Vec<u8>> {
    if get_model {
        let model_data = deps.storage.get(b"model").unwrap_or_default();
        Ok(model_data)
    } else {
        Ok(vec![])
    }
}
