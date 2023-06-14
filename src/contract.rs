#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;
use protobuf::Message;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GranterResponse, InstantiateMsg, QueryMsg};
use crate::state::GRANTER;
// Get Protos
include!("protos/mod.rs");
use CosmosAuthz::MsgExec;
use CosmosBankSend::Coin;
use CosmosBankSend::MsgSend;

use self::CosmosWithdrawDelegationReward::MsgWithdrawDelegatorReward;

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    GRANTER.save(deps.storage, &info.sender)?;

    Ok(Response::new()
        .add_attribute("contract", CONTRACT_NAME)
        .add_attribute("action", "instantiate")
        .add_attribute("granter", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::TransferFunds { to_address } => execute_transfer(deps, info, env, to_address),
        ExecuteMsg::WithdrawRewards { validator_address } => execute_withdraw_rewards(deps, info, env, validator_address),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Granter {} => to_binary(&query_granter(deps)?),
    }
}

pub fn query_granter(deps: Deps) -> StdResult<GranterResponse> {
    let granter = GRANTER.load(deps.storage)?;
    Ok(GranterResponse { granter })
}

pub fn execute_transfer(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    to_address: Addr,
) -> Result<Response, ContractError> {
    deps.api.addr_validate(&to_address.to_string())?;

    let mut send = MsgSend::new();
    send.from_address = info.sender.into_string();
    send.to_address = to_address.to_string();
    send.amount = vec![];
    for each_fund in info.funds {
        let mut coin = Coin::new();
        coin.amount = each_fund.amount.to_string();
        coin.denom = each_fund.denom;
        send.amount.push(coin);
    }

    let mut exec = MsgExec::new();
    exec.grantee = env.contract.address.to_string();
    exec.msgs = vec![send.to_any().unwrap()];
    let exec_bytes: Vec<u8> = exec.write_to_bytes().unwrap();

    let msg = CosmosMsg::Stargate {
        type_url: "/cosmos.authz.v1beta1.MsgExec".to_string(),
        value: Binary::from(exec_bytes),
    };
    Ok(Response::new()
        .add_attribute("method", "execute_authz_transfer")
        .add_message(msg))
}

pub fn execute_withdraw_rewards(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    validator_address: Addr,
) -> Result<Response, ContractError> {
    deps.api.addr_validate(&validator_address.to_string())?;

    let mut withdraw_rewards = MsgWithdrawDelegatorReward::new();
    withdraw_rewards.delegator_address = info.sender.to_string();
    withdraw_rewards.validator_address = validator_address.to_string();
    let mut exec = MsgExec::new();
    exec.grantee = env.contract.address.to_string();
    exec.msgs = vec![withdraw_rewards.to_any().unwrap()];
    let exec_bytes: Vec<u8> = exec.write_to_bytes().unwrap();

    let msg = CosmosMsg::Stargate {
        type_url: "/cosmos.authz.v1beta1.MsgExec".to_string(),
        value: Binary::from(exec_bytes),
    };
    
    Ok(Response::new()
        .add_attribute("method", "execute_authz_transfer")
        .add_message(msg))
}
