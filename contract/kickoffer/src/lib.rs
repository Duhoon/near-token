use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::store::Vector;
use near_sdk::{AccountId, Balance, Promise};
use near_sdk::{env, require, near_bindgen};

const INITIAL_BALANCE: Balance = 0;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct KickOffer{
    pub proposal_plans: Vector<AccountId>
}

#[near_bindgen]
impl KickOffer{
    #[private]
    pub fn create_proposal_plan_contract(prefix: AccountId, _code: Vec<u8>) -> Promise{
        let subaccount_id = AccountId::new_unchecked(
            format!("{}.{}", prefix, env::current_account_id())
        );
        Promise::new(subaccount_id)
            .create_account()
            .add_full_access_key(env::signer_account_pk())
            .transfer(1234)
    }
}