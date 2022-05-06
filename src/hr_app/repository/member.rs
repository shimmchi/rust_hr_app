use super::super::domain::department::list::DepartmentList;
use super::super::domain::member::MemberList;
use super::super::domain::person::list::PersonList;
use std::error::Error;

pub trait MemberRepository {
    fn read_all_data(
        &self,
        member_list: &mut MemberList,
        person_list: &PersonList,
        department_list: &DepartmentList,
    ) -> Result<(), Box<dyn Error>>;
    fn overwrite_all_data(&self, member_list: &mut MemberList) -> Result<(), Box<dyn Error>>;
}
