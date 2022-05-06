use super::super::domain::person::list::PersonList;
use std::error::Error;

pub trait PersonRepository {
    fn read_all_data(&self, person_list: &mut PersonList) -> Result<(), Box<dyn Error>>;
    fn overwrite_all_data(&self, person_list: &mut PersonList) -> Result<(), Box<dyn Error>>;
}
