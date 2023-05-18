pub mod vote_ext;
pub use crate::vote_ext::*;

const CODE: &[u8] = include_bytes!("../../target/wasm32-unknown-unknown/release/vote.wasm");

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{U128, U64};
use near_sdk::{AccountId, Balance, Gas, Promise, PromiseError};
use near_sdk::{env, log, near_bindgen};
use near_sdk::ext_contract;
use near_sdk::collections::{LookupSet, LookupMap};
use near_contract_standards::fungible_token::core::ext_ft_core;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct VoteController{
    communities: LookupSet<AccountId>,
    is_voting: LookupMap<AccountId, bool>
}

// 1 NEAR
const INITIAL_BALANCE: Balance = 5_000_000_000_000_000_000_000_000;
const TGAS: u64 = 1_000_000_000;

impl Default for VoteController{
    fn default()-> Self{
        Self{
            communities: LookupSet::new(b'i'),
            is_voting: LookupMap::new(b'i'),
        }
    }
}

#[near_bindgen]
impl VoteController{
    #[private]
    pub fn toggle_voting(&mut self, community_id: AccountId){
        
    }

    pub fn is_voting(&self, community_id: AccountId) -> bool{
        self.is_voting.get(&community_id).unwrap_or(false)
    }

    pub fn is_community(&self, community_id: AccountId) -> bool{
        self.communities.contains(&community_id)
    }

    pub fn add_community(&mut self, community_id: AccountId) -> bool{
        self.communities.insert(&community_id)
    }

    pub fn new_vote(&self, prefix: AccountId, community_id: AccountId) -> Promise{
        log!("Creating a new vote for {}", prefix.to_string());
        let subaccount_id = AccountId::new_unchecked(
            format!("{}.{}", prefix, env::current_account_id())
        );

        Promise::new(subaccount_id.clone())
            .create_account()
            .add_full_access_key(env::signer_account_pk())
            .transfer(INITIAL_BALANCE)
            .deploy_contract(CODE.to_vec())
        .then(
            ext_vote::ext(subaccount_id.clone())
            .with_static_gas(Gas(5*TGAS))
            .new(community_id)
        )
    }
}
