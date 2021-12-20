#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, BankMsg, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};
//use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{read_config, store_config, Config};

// version info for migration info
//const CONTRACT_NAME: &str = "crates.io:token-holding-contract";
//const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    store_config(deps.storage).save(&Config {
        owner: deps.api.addr_canonicalize(&msg.owner)?,
    })?;
    //set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Bank { amount, denom } => tokens(deps, info, amount, denom),
    }
}

pub fn tokens(
    deps: DepsMut,
    info: MessageInfo,
    amount: Option<Uint128>,
    denom: Option<String>,
) -> Result<Response, ContractError> {
    let config = read_config(deps.storage)?;
    if deps.api.addr_canonicalize(info.sender.as_str())? != config.owner {
        return Err(ContractError::Unauthorized {});
    }

    match amount {
        Some(amount) => match denom {
            Some(denom) => Ok(Response::new().add_message(BankMsg::Send {
                amount: vec![Coin::new(amount.u128(), denom)],
                to_address: info.sender.into_string(),
            })),
            None => {
                Err(ContractError::TokenErr {
                    denom: "token".to_string(),
                })
            }
        },
        None => Ok(Response::default()),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
    }
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = read_config(deps.storage)?;
    Ok(ConfigResponse {
        owner: deps.api.addr_humanize(&config.owner)?.into_string(),
    })
}
