#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{config, CONFIG};

const CONTRACT_NAME: &str = "crates.io:cw-counter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let Config = config {
        last_incrimenter: info.sender.clone(),
        counter: msg.val.clone(),
    };
    CONFIG.save(deps.storage, &Config)?;
    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("admin", info.sender.to_string())
        .add_attribute("Counter", msg.val.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::IncrementCounter { val } => increment_num(deps, info, val),
    }
}

pub fn increment_num(
    deps: DepsMut,
    info: MessageInfo,
    val: u64,
) -> Result<Response, ContractError> {
    /*  let mut Config = CONFIG.load(deps.storage)?;
    // Config.counter = val;

    CONFIG.save(deps.storage, &Config);

    both do the same
    */
    CONFIG.update(deps.storage, |mut config| -> StdResult<_> {
        config.counter = val;
        Ok(config)
    })?;
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCounter {} => get_counter(deps),
    }
}

pub fn get_counter(deps: Deps) -> StdResult<Binary> {
    let config = CONFIG.load(deps.storage)?;
    to_json_binary(&config.counter)
}

#[cfg(test)]
mod tests {}
