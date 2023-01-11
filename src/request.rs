use crate::campaign::CampaignInput;
use near_sdk::{
    serde::{Deserialize, Serialize},
    serde_json,
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Request {
    kind: Kind,
    campaign: Option<CampaignInput>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum Kind {
    CreateCampaign,
    Unknown,
}

impl Request {
    pub fn from_json(s: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(s)
    }

    pub fn kind(&self) -> &Kind {
        &self.kind
    }

    pub fn campaign(&self) -> Option<CampaignInput> {
        self.campaign.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use near_sdk::{json_types::U128, log};

    use super::*;

    #[test]
    fn test_request_from_json() {
        let request = Request::from_json(
            r#"{"kind":"CreateCampaign", "campaign":{"name":"Tin callings", "goal":"1000000000000000000000000", "incoming_token_id":"dev-1610107640000-10000000000000", "outgoing_token_id":"dev-1610107640000-10000000000000", "end_time":"1610107640000", "start_time":"1610107640000"}}"#,
        ).unwrap();

        // assert_eq!(request.kind(), Kind::CreateCampaign);
        log!("request.kind: {:?}", request.kind());
        assert_eq!(request.campaign().unwrap().name, "Tin callings");
        assert_eq!(
            request.campaign().unwrap().goal,
            U128::from(1000000000000000000000000)
        );
        assert_eq!(
            request.campaign().unwrap().incoming_token_id.to_string(),
            "dev-1610107640000-10000000000000"
        );
        assert_eq!(
            request.campaign().unwrap().outgoing_token_id.to_string(),
            "dev-1610107640000-10000000000000"
        );
        assert_eq!(request.campaign().unwrap().end_time.0, 1610107640000);
        assert_eq!(request.campaign().unwrap().start_time.0, 1610107640000);
    }
}
