use cosmwasm_std::{Deps, StdResult};

pub fn infer(
    deps: Deps,
    input: InputData,
) -> StdResult<OutputData> {
    let model: Model = bincode::deserialize(&deps.storage.get(b"model").ok_or(StdError::not_found("Model"))?)?;
    let output = model.weights.iter().zip(input.inputs.iter()).map(|(w, i)| w * i).collect::<Vec<f64>>();
    Ok(OutputData { outputs: output })
}
