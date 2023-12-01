use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;
use fish_common::MintData;

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Option<Addr>,
    pub collection: String,
    pub mint_denom: String,
    pub mint_data:  Vec<MintData>,
}

#[cw_serde]
pub enum ExecuteMsg {
    Mint {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(CollectionResponse)]
    Collection {},
}

#[cw_serde]
pub struct CollectionResponse {
    pub address: String,
}