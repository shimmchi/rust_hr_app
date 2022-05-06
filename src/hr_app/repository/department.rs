use super::super::domain::department::list::DepartmentList;
use std::error::Error;

pub trait DepartmentRepository {
    fn read_all_data(&self, department_list: &mut DepartmentList) -> Result<(), Box<dyn Error>>;
    fn overwrite_all_data(
        &self,
        department_list: &mut DepartmentList,
    ) -> Result<(), Box<dyn Error>>;
}
