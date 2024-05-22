use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

pub fn init_model(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    weights: Vec<f64>,
) -> StdResult<Response> {
    let model = Model { weights };
    // 저장소에 모델 저장
    deps.storage.set(b"model", &bincode::serialize(&model)?);
    Ok(Response::new().add_attribute("action", "init_model"))
}
