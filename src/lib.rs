use near_sdk::env;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    near_bindgen, AccountId,
};
use std::collections::HashMap;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct CrowdFunding {
    campaign: HashMap<AccountId, Campaign>,
}

#[derive(BorshDeserialize, BorshSerialize, Clone)]
pub struct Campaign {
    name: String,
    description: String,
    goal: u128,
    deadline: u64,
    raised: u128,
    is_active: bool,
    owner_id: AccountId,
    contributors: Vec<AccountId>,
    token: AccountId,
}

#[near_bindgen]
impl CrowdFunding {
    pub fn create_campaign(
        &mut self,
        name: String,
        description: String,
        goal: u128,
        deadline: u64,
        token: AccountId,
    ) {
        let account_id = env::signer_account_id();
        let campaign = Campaign {
            name,
            description,
            goal,
            deadline,
            raised: 0,
            is_active: true,
            owner_id: account_id.clone(),
            contributors: vec![],
            token,
        };
        self.campaign.insert(account_id, campaign);
    }

    pub fn get_campaign(&self, account_id: AccountId) -> Option<Campaign> {
        self.campaign.get(&account_id).cloned()
    }
}
