use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedSet, UnorderedMap};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Birthday {
    pub account_id: String,
    pub name_dates: UnorderedMap<String, String>,
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
        self.name_dates.insert(&name, &date);
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
         self.name_dates.remove(&name);

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

     pub fn get_name(&self, name: String) -> Option<String> {
         // return all the dates of folks with a birthday for this name
         self.name_dates.get(&name)
     }

     pub fn get_all_names(&self) -> Vec<String> {
         self.name_dates.keys_as_vector().to_vec()
     }

     pub fn get_all_dates(&self) -> Vec<String> {
         self.date_names.keys_as_vector().to_vec()
     }

     pub fn get_all(&self) -> Vec<(String, String)> {
         self.name_dates.to_vec()
     }
}
