use super::super::name_type::NameType;
use std::error::Error;

// Departmentの名前を格納する
// フィールドは１つ
// 最大長と最短長は固定値
pub struct DepartmentName {
    name: NameType,
}

impl DepartmentName {
    const MIN_LENGTH: u8 = 1;
    const MAX_LENGTH: u8 = 100;

    pub fn new(name: &str) -> Result<DepartmentName, Box<dyn Error>> {
        let name = NameType::new(name, Self::MIN_LENGTH, Self::MAX_LENGTH)?;
        return Ok(DepartmentName { name: name });
    }

    // 画面表示用に文字列を出力する
    pub fn value(&self) -> String {
        return String::from(self.name.value());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_ok() {
        assert!(DepartmentName::new("Sales").is_ok());
    }

    #[test]
    fn new_min_boundary_ok() {
        assert!(DepartmentName::new("a").is_ok());
    }

    #[test]
    fn new_too_short_err() {
        assert!(DepartmentName::new("").is_err());
    }

    #[test]
    fn new_max_boundary_ok() {
        assert!(DepartmentName::new("0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789").is_ok());
    }

    #[test]
    fn new_too_long_err() {
        assert!(DepartmentName::new("01234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890").is_err())
    }

    #[test]
    fn value_ok() {
        let ans = String::from("Sales");
        let department_name = DepartmentName::new("Sales").unwrap();
        assert_eq!(department_name.value(), ans);
    }
}
