const CODE: &[u8] = include_bytes!("../../target/wasm32-unknown-unknown/release/hello_near.wasm");

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{U128, U64};
use near_sdk::{AccountId, Balance, EpochHeight, Gas, Promise, PromiseError, PanicOnDefault};
use near_sdk::{env, log, near_bindgen};
use near_sdk::ext_contract;
use near_sdk::collections::{LookupSet, LookupMap};
use near_contract_standards::fungible_token::core::ext_ft_core;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct VoteController{
    communities: LookupSet<AccountId>,
    is_voting: LookupMap<AccountId, bool>
}

// 1 NEAR
const INITIAL_BALANCE: Balance = 1_000_000_000_000_000_000_000_000;

#[near_bindgen]
impl VoteController{
    #[init]
    pub fn new()->Self{
        Self{
            communities: LookupSet::new(b'i'),
            is_voting: LookupMap::new(b'i'),
        }
    }

    pub fn is_voting(&self, community_id: AccountId) -> bool{
        self.is_voting.get(&community_id).unwrap_or(false)
    }

    pub fn is_community(&self, community_id: AccountId) -> bool{
        self.communities.contains(&community_id)
    }

    pub fn new_vote(&self, prefix: AccountId) -> Promise{
        log!("Creating a new vote for {}", prefix.to_string());
        let subaccount_id = AccountId::new_unchecked(
            format!("{}.{}", prefix, env::current_account_id())
        );

        Promise::new(subaccount_id)
            .create_account()
            .add_full_access_key(env::signer_account_pk())
            .transfer(INITIAL_BALANCE)
            .deploy_contract(CODE.to_vec())
    }
}
