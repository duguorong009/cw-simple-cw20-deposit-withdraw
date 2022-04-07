#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
use cw20::Cw20ReceiveMsg;

use crate::error::ContractError;
use crate::msg::{ ExecuteMsg, InstantiateMsg, QueryMsg, WithdrawMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:trade-cw20";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    Ok(Response::new()
        .add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Withdraw (msg) => withdraw_cw20(deps, info, msg),
        ExecuteMsg::Receive(msg) => deposit_cw20(deps, info, msg),
    }
}

fn withdraw_cw20(deps: DepsMut, info: MessageInfo, msg: WithdrawMsg) -> Result<Response, ContractError> {
    // Get the params from WithdrawMsg

    // Validations

    // Handle the real "withdraw"

    Ok(Response::default())
}


fn deposit_cw20(deps: DepsMut, info: MessageInfo, cw20_msg: Cw20ReceiveMsg) -> Result<Response, ContractError> {
    // Validate if the message comes from "cw20" token

    // Deseralize the message for the params

    // Handle the real "deposit".

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        // TODO
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

}
