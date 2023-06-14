use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

#[cw_serde]
pub struct InstantiateMsg {
}

#[cw_serde]
pub enum ExecuteMsg {
    TransferFunds {
        to_address: Addr,
    },
    WithdrawRewards{
        validator_address: Addr,
    }
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GranterResponse)]
    Granter {},
}

#[cw_serde]
pub struct GranterResponse{
    pub granter: Addr,
}