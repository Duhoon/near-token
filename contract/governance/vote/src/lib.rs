mod vote;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{U128, U64};
use near_sdk::{env, near_bindgen, AccountId, Balance, EpochHeight, Gas, Promise, PromiseError, PanicOnDefault};
use near_sdk::ext_contract;
use near_sdk::collections::LookupMap;
use near_contract_standards::fungible_token::core::ext_ft_core;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct VoteManager{
    communities: Vec<AccountId>,
    is_voting: Vec<bool>,
}

#[near_bindgen]
impl VoteManger{
    #[init]
    pub fn new()->Self{
        Self{
            communities: Vec::new(),
            is_voting: Vec::new(),
        }
    }

    pub fn is_voting(&self, community_id: U128) -> bool{
        is_voting[u128(community_id.0)]
    }

    pub fn get_community(&self, community_id: U128) -> bool{
        communities[u128(community_id.0)]
    }
}
