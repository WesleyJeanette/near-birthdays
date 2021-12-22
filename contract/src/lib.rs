/*
 *
 */

// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, setup_alloc};
use near_sdk::collections::{UnorderedMap};

pub use crate::birthday::*;

pub mod birthday;

setup_alloc!();


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct BirthdayContract {
    records: UnorderedMap<String, Birthday>,
}

impl Default for BirthdayContract {
  fn default() -> Self {
      env::panic(b"Birthdays contract should be initialized before using")
  }
}

#[near_bindgen]
impl BirthdayContract {
    #[init]
    pub fn new() -> Self {
        assert!(env::state_read::<Self>().is_none(), "Contract is alraedy initialized");

        BirthdayContract {
            records: UnorderedMap::new(b"a".to_vec()),
        }
    }

    pub fn add(&mut self, name: String, date: String) {
        // validate the date
        //
        let account_id = env::current_account_id();
        match self.records.get(&account_id) {
            Some(mut b) => {
                b.add(name, date);
                self.records.insert(&account_id, &b);
                return
            }
            None => {
                let id = account_id.clone();
                let mut b = Birthday::new(id);
                b.add(name, date);
                self.records.insert(&account_id, &b);
                return
            }
        }
    }

    pub fn remove(&mut self, name: String, date: String) {
        // make sure both name and date are here so you don't remove the wrong
        // joe smith if there exist more than one.
        let account_id = env::current_account_id();
        match self.records.get(&account_id) {
            Some(mut b) => {
                b.remove(name, date);
                self.records.insert(&account_id, &b);
            },
            None => {
                // account does not exist...
            }
        }
    }

    pub fn get_birthdays_for_date(&self, date: String) -> Option<Vec<String>> {
        // return all the names of folks with a birthday for this date
        let account_id = env::current_account_id();
        match self.records.get(&account_id) {
            Some(b) => {
                match b.get_date(date) {
                    Some(r) => {
                        return Some(r.to_vec())
                    },
                    None => {
                        return None
                    },
                }
            },
            None => {
                return None
                // date does not exist
            }
        }
    }

    pub fn get_birthdays_for_name(&self, name: String) -> Option<Vec<String>> {
        // return all the dates of folks with a birthday for this name
        let account_id = env::current_account_id();
        match self.records.get(&account_id) {
            Some(b) => {
                match b.get_name(name) {
                    Some(r) => {
                        return Some(r.to_vec())
                    },
                    None => {
                        return None
                    },
                }
            },
            None => {
                return None
                // date does not exist
            }
        }
    }

    pub fn get_all_birthdays_by_name(&self) -> Option<Vec<String>> {
        // return all the dates of folks with a birthday for this name
        let account_id = env::current_account_id();
        match self.records.get(&account_id) {
            Some(b) => {
                return Some(b.get_all_names())
            },
            None => {
                return None
                // date does not exist
            }
        }
    }

    pub fn get_all_birthdays_by_date(&self) -> Option<Vec<String>> {
        // return all the dates of folks with a birthday for this name
        let account_id = env::current_account_id();
        match self.records.get(&account_id) {
            Some(b) => {
                return Some(b.get_all_dates())
            },
            None => {
                return None
                // date does not exist
            }
        }
    }

    // pub fn get_birthdays(&self, name: Option<String>, date: Option<String>) -> Option<Vec<String>> {
    //     // return all the dates of folks with a birthday for this name
    //     let account_id = env::current_account_id();
    //     let record = self.records.get(&account_id);
    //     if is_some(record) {
    //         if is_some(name) {
    //             match Some(record).get_name(Some(name)) {
    //                 Some(r) => {
    //                     return Some(r.to_vec())
    //                 },
    //                 None => {
    //                     return None
    //                 },
    //             }
    //         }
    //         // None => {
    //         //     return None
    //         //     // date does not exist
    //         // }
    //     }
    // }
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
    fn new_birthday() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Birthdays::new();
        contract.add_birthday("Billy Joel".to_string(), "May 9th".to_string());
        assert_eq!(
            "Billy Joel".to_string(),
            contract.get_birthdays_date("May 9th".to_string())
        );
        assert_eq!(
            "May 9th".to_string(),
            contract.get_birthdays_Name("Billy Joel".to_string())
        );
    }

    // #[test]
    // fn duplicate_name() {
    //     let context = get_context(vec![], true);
    //     testing_env!(context);
    //     let contract = Birthdays::new();
    //     // this test did not call set_greeting so should return the default "Hello" greeting
    //     assert_eq!(
    //         "Hello".to_string(),
    //         contract.get_greeting("francis.near".to_string())
    //     );
    // }
}
