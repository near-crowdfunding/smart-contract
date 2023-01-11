use crate::{
    campaign::{Campaign, CampaignInput},
    request::Request,
};
use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::{LazyOption, UnorderedMap},
    near_bindgen, AccountId, PromiseOrValue,
    __private::BorshIntoStorageKey,
    env::panic_str,
    ext_contract, log,
};
use near_sdk::{
    env,
    json_types::{U128, U64},
    PanicOnDefault,
};
use request::Kind;

mod campaign;
mod request;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct NearIDO {
    campaigns: UnorderedMap<U128, Campaign>,
    total_campaigns: U128,
    total_contributions: U128,
}

#[derive(BorshSerialize)]
pub enum StorageKeys {
    Campaign,
}
impl BorshIntoStorageKey for StorageKeys {}

#[near_bindgen]
impl NearIDO {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Already initialized");

        Self {
            campaigns: UnorderedMap::new(StorageKeys::Campaign),
            total_campaigns: U128::from(0),
            total_contributions: U128::from(0),
        }
    }

    #[private]
    pub fn internal_create_campaign(
        &mut self,
        name: String,
        goal: U128,
        incoming_token_id: AccountId,
        outgoing_token_id: AccountId,
        amount: U128,
        start_time: U64,
        end_time: U64,
    ) -> PromiseOrValue<U128> {
        assert!(
            env::is_valid_account_id(incoming_token_id.as_bytes()),
            "Invalid incoming token ID"
        );
        assert!(
            env::is_valid_account_id(outgoing_token_id.as_bytes()),
            "Invalid outgoing token ID"
        );
        assert!(
            env::is_valid_account_id(env::predecessor_account_id().as_bytes()),
            "Invalid owner ID"
        );
        assert!(
            env::block_timestamp() < start_time.into(),
            "Start time must be in the future"
        );
        assert!(
            start_time.0 < end_time.0,
            "End time must be after start time"
        );
        assert!(
            amount.0 > 0,
            "You must contribute at least 1 token to create a campaign"
        );
        assert!(
            goal.0 > 0,
            "You must set a goal greater than 0 to create a campaign"
        );
        assert!(name.len() > 0, "You must set a name for your campaign");
        assert!(
            name.len() <= 64,
            "Campaign name must be less than 64 characters"
        );

        let caller = env::predecessor_account_id();
        let campaign_id = self.total_campaigns;
        let campaign = Campaign {
            name,
            goal,
            raised: amount,
            is_active: true,
            owner_id: caller,
            contributors: vec![],
            incoming_token_id,
            outgoing_token_id,
            end_time: end_time.into(),
            start_time: start_time.into(),
        };

        self.campaigns.insert(&campaign_id, &campaign);
        self.total_campaigns.0 += 1;
        self.total_contributions.0 += amount.0;

        log!("Campaign created: {}", campaign_id.0);

        PromiseOrValue::Value(campaign_id)
    }

    pub fn get_campaign(&self, campaign_id: U128) -> LazyOption<Campaign> {
        LazyOption::new(
            StorageKeys::Campaign,
            Some(&self.campaigns.get(&campaign_id).unwrap()),
        )
    }

    pub fn change_campaign_status(&mut self, campaign_id: U128, is_active: bool) {
        let mut campaign = self
            .campaigns
            .get(&campaign_id)
            .expect("Campaign not found");
        assert_eq!(
            env::predecessor_account_id(),
            campaign.owner_id,
            "Only the owner can change the campaign status"
        );
        campaign.is_active = is_active;
        self.campaigns.insert(&campaign_id, &campaign);
    }

    // Turn token_id into campaign_id
    fn cancel(&mut self, campaign_id: U128) {}
    fn pledge(&mut self, campaign_id: U128) {}
    fn unpledge(&mut self, campaign_id: U128) {}
    fn withdraw(&mut self, campaign_id: U128) {}
    fn claim(&mut self, campaign_id: U128) {}
    fn refund(&mut self, campaign_id: U128) {}
}

// Defining cross-contract interface. This allows to create a new promise.
#[ext_contract(ext_self)]
pub trait ValueReturnTrait {
    fn value_please(&self, amount_to_return: String) -> PromiseOrValue<U128>;
}
#[ext_contract(ext_ft_contract)]
pub trait FungibleTokenCore {
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>);
}

#[near_bindgen]
impl FungibleTokenReceiver for NearIDO {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        log!("ft_on_transfer: {}, {}, {}", sender_id, amount.0, msg);

        let request = Request::from_json(&msg)
            .map_err(|e| {
                log!("Error parsing request: {}", e);
                panic_str("Error parsing request");
            })
            .unwrap();

        match request.kind() {
            Kind::CreateCampaign => {
                let campaign = request.campaign().expect("Campaign not found");

                let CampaignInput {
                    name,
                    goal,
                    incoming_token_id,
                    outgoing_token_id,
                    start_time,
                    end_time,
                    ..
                } = campaign;

                log!(
                    "Campaign -> {}, {}, {}, {}, {}, {}",
                    name,
                    goal.0,
                    incoming_token_id,
                    outgoing_token_id,
                    start_time.0,
                    end_time.0,
                );

                self.internal_create_campaign(
                    name,
                    goal,
                    incoming_token_id,
                    outgoing_token_id,
                    amount,
                    start_time.into(),
                    end_time.into(),
                );
            }
            _ => env::panic_str("Invalid request"),
        }

        PromiseOrValue::Value(U128::from(0))
    }
}
