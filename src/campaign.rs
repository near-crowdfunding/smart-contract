use near_sdk::{
    borsh,
    json_types::{U128, U64},
    serde::{Deserialize, Serialize},
    AccountId,
};

#[derive(borsh::BorshDeserialize, borsh::BorshSerialize, Clone, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Campaign {
    pub name: String,
    pub goal: U128,
    pub raised: U128,
    pub is_active: bool,
    pub owner_id: AccountId,
    pub contributors: Vec<AccountId>,
    pub incoming_token_id: AccountId,
    pub outgoing_token_id: AccountId,
    pub end_time: u64,
    pub start_time: u64,
}

#[derive(
    borsh::BorshDeserialize, borsh::BorshSerialize, Clone, Serialize, Deserialize, Debug, PartialEq,
)]
#[serde(crate = "near_sdk::serde")]
pub struct CampaignInput {
    pub name: String,
    pub goal: U128,
    #[serde(rename = "incoming_token_id")]
    pub incoming_token_id: AccountId,
    #[serde(rename = "outgoing_token_id")]
    pub outgoing_token_id: AccountId,
    #[serde(rename = "end_time")]
    pub end_time: U64,
    #[serde(rename = "start_time")]
    pub start_time: U64,
}
