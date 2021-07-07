// TODO 1: fn cr current_balance: (), description: (), duration: (), goal: (), state: (), title: () eate_project
// TODO 2: fn returnAllProjects
// TODO 3: fn contribute
// TODO 5: fn checkIfFundingCompleteOrExpired
// TODO 6: fn payout
// TODO 7: fn getRefund
// TODO 8: fn getDetails
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, UnorderedSet};
use near_sdk::{env, near_bindgen};
use std::fmt::Debug;

near_sdk::setup_alloc!();
#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq)]
pub enum State {
    Expired,
    Live,
    Successful,
}
// impl BorshIntoStorageKey for State {}
#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq)]
pub struct Project {
    current_balance: u128,
    description: String,

    duration: u128,
    goal: u128,
    state: State,
    title: String,
}

// impl BorshIntoStorageKey for Project {}
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Projects {
    list: UnorderedMap<String, Project>,
}

impl Default for Projects {
    fn default() -> Self {
        env::panic(b"The contract should be initialized before usage")
    }
}

#[near_bindgen]
impl Projects {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");
        Self {
            list: UnorderedMap::new(b"s".to_vec()),
        }
    }
    pub fn create_project(
        &mut self,
        title: String,
        description: String,
        duration: u128,
        goal: u128,
    ) {
        let creator = env::predecessor_account_id();
        self.list.insert(
            &creator,
            &Project {
                current_balance: 0,
                description,
                duration,
                goal,
                state: State::Live,
                title,
            },
        );
    }

    pub fn get_details(&self, creator: String) -> Option<Project> {
        return self.list.get(&creator);
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use std::u128;

    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 0,
        }
    }

    #[test]
    fn create_project() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Projects::new();
        contract.create_project(
            "Test Project".to_string(),
             "This is the description of the project".to_string(),
             123,
             1234,
        );
        assert_eq!(
            Project {
                current_balance: 0,
                description: "This is the description of the project".to_string(),
                duration: 123 as u128,
                goal: 1234 as u128,
                state: State::Live,
                title: "Test Project".to_string(),
            },
            contract.get_details(env::predecessor_account_id()).unwrap()
        );
    }
}
