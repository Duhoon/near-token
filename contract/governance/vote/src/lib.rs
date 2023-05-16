use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{U128, U64};
use near_sdk::{env, near_bindgen, AccountId, Balance, EpochHeight, Gas, Promise, PromiseError, PanicOnDefault};
use near_sdk::ext_contract;
use near_sdk::collections::LookupMap;
use near_contract_standards::fungible_token::core::ext_ft_core;

type WrappedTimestamp = U64;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Vote{
    votes: LookupMap<Account, Balance>,
    total_voted_stake: Balance,
    result: Option<WrappedTimestamp>,
    last_epoch_height: EpochHeight,
    governanace_accountId: AccountId,
}

impl Default for VotingContract{
    fn default() -> Self {
        env::panic(b"Voting Contract should be initialized before usage");
    }
}

#[ext_contract(ext_vote)]
pub trait extVote{
    pub fn ping(&mut self);
    pub fn vote(&mut self, is_vote: bool);
    pub fn get_result(&self) -> Option<WrappedTimestamp>;
    pub fn get_total_voted_stake(&self) -> (U128, U128);
    pub fn get_votes(&self) -> LookupMap<AccountId, U128>;
}


#[near_bindgen]
impl Vote {
    #[init]
    pub fn new() -> Self{
        assert!(!env:state_exists(), "The contract is already initialized");
        VotingContract {
            votes: LookupMap::new(b"d"),
            total_voted_stake: 0,
            result: None,
            last_epoch_height: 0,
        }
    }

    pub fn ping(&mut self){

    }

    pub fn vote(&mut self, is_vote: bool){
        self.ping();
        if self.result.is_some(){
            return;
        }
        let account_id = env::predecessor_account_id();
        ext_ft_core::ft_balance_of(self.governanace_accountId.copy())
        .with_static_gas(Gas)
        ;
    }

    pub fn get_result(&self) -> Option<WrappedTimestamp>{

    }

    pub fn get_total_voted_stake(&self) -> (U128, U128){

    }

    pub fn get_votes(&self) -> LookupMap<AccountId, U128>{

    }
}