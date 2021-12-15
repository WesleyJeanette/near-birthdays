use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedSet, UnorderedMap};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Birthday {
    pub account_id: String,
    pub name_dates: UnorderedMap<String, UnorderedSet<String>>,
    pub date_names: UnorderedMap<String, UnorderedSet<String>>,
}

impl Birthday {
    pub fn new(user: String) -> Self {
        let set_name = format!("{}_name", &user);
        let set_date = format!("{}_date", &user);

        Birthday {
            account_id: user,
            name_dates: UnorderedMap::new(set_name.into_bytes()),
            date_names: UnorderedMap::new(set_date.into_bytes()),
        }
    }

    pub fn add(&mut self, name: String, date: String) {
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

     pub fn remove(&mut self, name: String, date: String) {
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

     pub fn get_date(&self, date: String) -> Option<Vec<String>> {
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

     pub fn get_name(&self, name: String) -> Option<Vec<String>> {
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
}
