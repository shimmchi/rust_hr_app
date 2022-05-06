pub mod id;
pub mod list;
pub mod name;

use name::PersonName;
use std::error::Error;

// 個人に関する情報を格納する
// 情報として持っているのは名前のみ
pub struct Person {
    name: PersonName,
}

impl Person {
    pub fn new(last_name: &str, first_name: Option<&str>) -> Result<Person, Box<dyn Error>> {
        let name = PersonName::new(last_name, first_name)?;
        return Ok(Person { name: name });
    }

    // 現在持っている情報が名前のみなので、情報の出力メソッド名もnameとした
    // 名前以外の情報も含むように鳴ればほかにinfoメソッドなどを用意する必要がある
    pub fn name(&self) -> String {
        let name = self.name.value();
        return name;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_person_and_get_name() {
        let person = Person::new("Shimomichi", Some("Yuta")).unwrap();
        assert_eq!(person.name(), String::from("Shimomichi Yuta"));
    }
}
