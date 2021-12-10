/*
 *
 */

// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, setup_alloc};
use near_sdk::collections::{UnorderedSet, UnorderedMap};

setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Birthdays {
    name_dates: UnorderedMap<String, UnorderedSet<String>>,
    date_names: UnorderedMap<String, UnorderedSet<String>>,
}

impl Default for Birthdays {
  fn default() -> Self {
      env::panic(b"Birthdays contract should be initialized before using")
  }
}

#[near_bindgen]
impl Birthdays {
    #[init]
    pub fn new() -> Self {
        assert!(env::state_read::<Self>().is_none(), "Contract is alraedy initialized");

        Birthdays {
            name_dates: UnorderedMap::new(b"a".to_vec()),
            date_names: UnorderedMap::new(b"b".to_vec()),
        }
    }

    pub fn add_birthday(&mut self, name: String, date: String) {
        // validate the date
        //
        match self.name_dates.get(&name) {
            Some(mut record) => {
                record.insert(&date);
                self.name_dates.insert(&name, &record);
            },
            None => {
                let set_name = name.clone();
                let mut record = UnorderedSet::new(set_name.into_bytes());
                record.insert(&date);
                self.name_dates.insert(&name, &record);
            }
        }
        // the date to names mapping
        match self.date_names.get(&date) {
            Some(mut record) => {
                record.insert(&name);
                self.date_names.insert(&date, &record);
            },
            None => {
                let set_date = date.clone();
                let mut record = UnorderedSet::new(set_date.into_bytes());
                record.insert(&name);
                self.date_names.insert(&date, &record);
            }
        }
    }

    pub fn remove_birthday(&mut self, name: String, date: String) {
        // make sure both name and date are here so you don't remove the wrong
        // joe smith if there exist more than one.
        match self.name_dates.get(&name) {
            Some(mut record) => {
                record.remove(&date);
                self.name_dates.insert(&name, &record);
            },
            None => {
                // name does not exist...
            }
        }
        // the date to names mapping
        match self.date_names.get(&date) {
            Some(mut record) => {
                record.remove(&name);
                self.date_names.insert(&date, &record);
            },
            None => {
                // date does not exist
            }
        }
    }

    pub fn get_birthdays_date(&self, date: String) -> Option<Vec<String>> {
        // return all the names of folks with a birthday for this date
        match self.date_names.get(&date) {
            Some(record) => {
                return Some(record.to_vec())
            },
            None => {
                return None
                // date does not exist
            }
        }
    }

    pub fn get_birthdays_name(&self, name: String) -> Option<Vec<String>> {
        // return all the dates of folks with a birthday for this name
        match self.name_dates.get(&name) {
            Some(record) => {
                return Some(record.to_vec())
            },
            None => {
                return None
            }
        }
    }

    // pub fn get_all_birthdays(&self) -> Vec<(String,String)> {
    //     self.name_dates.to_vec()
    // }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 *
 * To run from contract directory:
 * cargo test -- --nocapture
 *
 * From project root, to run in combination with frontend tests:
 * yarn test
 *
 */
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // mock the context for testing, notice "signer_account_id" that was accessed above from env::
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
            epoch_height: 19,
        }
    }

    #[test]
    fn set_then_get_greeting() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Welcome::default();
        contract.set_greeting("howdy".to_string());
        assert_eq!(
            "howdy".to_string(),
            contract.get_greeting("bob_near".to_string())
        );
    }

    #[test]
    fn get_default_greeting() {
        let context = get_context(vec![], true);
        testing_env!(context);
        let contract = Welcome::default();
        // this test did not call set_greeting so should return the default "Hello" greeting
        assert_eq!(
            "Hello".to_string(),
            contract.get_greeting("francis.near".to_string())
        );
    }
}
