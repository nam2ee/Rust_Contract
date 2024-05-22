use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response, StdResult, to_binary, Binary, Deps};
use crate::zkp::{generate_parameters, generate_proof, verify_proof};
use pairing::bn256::Fr;

// Init function to store ZKP parameters
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    let (params, pvk) = generate_parameters()?;
    deps.storage.set(b"params", &bincode::serialize(&params)?);
    deps.storage.set(b"pvk", &bincode::serialize(&pvk)?);
    Ok(Response::new().add_attribute("method", "instantiate"))
}

// Execute function to generate a proof
#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::GenerateProof { input, output } => {
            let params: groth16::Parameters<Bn256> = bincode::deserialize(&deps.storage.get(b"params").ok_or(StdError::not_found("params"))?)?;
            let input_fr = Fr::from_str(&input).map_err(|_| StdError::generic_err("Invalid input"))?;
            let output_fr = Fr::from_str(&output).map_err(|_| StdError::generic_err("Invalid output"))?;
            let proof = generate_proof(&params, input_fr, output_fr)?;
            deps.storage.set(b"proof", &bincode::serialize(&proof)?);
            Ok(Response::new().add_attribute("method", "generate_proof"))
        }
    }
}

// Query function to verify a proof
#[entry_point]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::VerifyProof { input, output } => {
            let pvk: groth16::PreparedVerifyingKey<Bn256> = bincode::deserialize(&deps.storage.get(b"pvk").ok_or(StdError::not_found("pvk"))?)?;
            let proof: groth16::Proof<Bn256> = bincode::deserialize(&deps.storage.get(b"proof").ok_or(StdError::not_found("proof"))?)?;
            let input_fr = Fr::from_str(&input).map_err(|_| StdError::generic_err("Invalid input"))?;
            let output_fr = Fr::from_str(&output).map_err(|_| StdError::generic_err("Invalid output"))?;
            let is_valid = verify_proof(&pvk, proof, input_fr, output_fr)?;
            to_binary(&is_valid)
        }
    }
}
