pub mod id;
pub mod list;
pub mod name;

use name::DepartmentName;
use std::error::Error;

// 部署に関する情報を格納する
// 情報として持っているのは名前のみ
pub struct Department {
    name: DepartmentName,
}

impl Department {
    pub fn new(name: &str) -> Result<Department, Box<dyn Error>> {
        let name = DepartmentName::new(name)?;
        return Ok(Department { name: name });
    }

    pub fn name(&self) -> String {
        return self.name.value();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_department_and_get_name() {
        let department = Department::new("Sales").unwrap();
        assert_eq!(department.name(), String::from("Sales"));
    }
}
