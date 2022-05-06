use super::super::name_type::NameType;
use std::error::Error;

// Personの名前を格納する
// 姓と名のフィールドを持つ
// 名の入力は任意
// 姓と名それぞれ最大長と最短超は固定値
pub struct PersonName {
    last_name: NameType,
    first_name: Option<NameType>,
}

impl PersonName {
    const MIN_LENGTH: u8 = 1;
    const MAX_LENGTH: u8 = 60;

    pub fn new(last_name: &str, first_name: Option<&str>) -> Result<PersonName, Box<dyn Error>> {
        let last_name = NameType::new(last_name, Self::MIN_LENGTH, Self::MAX_LENGTH)?;
        let first_name = match first_name {
            Some(first_name) => Some(NameType::new(
                first_name,
                Self::MIN_LENGTH,
                Self::MAX_LENGTH,
            )?),
            None => None,
        };

        return Ok(PersonName {
            last_name: last_name,
            first_name: first_name,
        });
    }

    // 画面表示用に姓と名を連結して文字列を返す
    pub fn value(&self) -> String {
        let last_name = self.last_name.value();
        match &self.first_name {
            Some(first_name) => return format!("{} {}", last_name, first_name.value()),
            None => return format!("{}", last_name),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_ok() {
        assert!(PersonName::new("Shimomichi", Some("Yuta")).is_ok());
    }

    #[test]
    fn new_last_name_too_short_err() {
        assert!(PersonName::new("", Some("Yuta")).is_err());
    }

    #[test]
    fn new_last_name_too_long_err() {
        assert!(PersonName::new(
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            Some("Yuta"),
        )
        .is_err());
    }

    #[test]
    fn new_first_name_too_short_err() {
        assert!(PersonName::new("Shimomichi", Some("")).is_err());
    }

    #[test]
    fn new_first_name_too_long_err() {
        assert!(PersonName::new(
            "Shimomichi",
            Some("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
        )
        .is_err());
    }

    #[test]
    fn new_no_first_name_ok() {
        assert!(PersonName::new("Shimomichi", None).is_ok());
    }

    #[test]
    fn fullname_ok_1() {
        let ans = String::from("Shimomichi Yuta");
        let person_name = PersonName::new("Shimomichi", Some("Yuta")).unwrap();
        assert_eq!(person_name.value(), ans);
    }

    #[test]
    fn fullname_ok_2() {
        let ans = String::from("Shimomichi");
        let person_name = PersonName::new("Shimomichi", None).unwrap();
        assert_eq!(person_name.value(), ans);
    }
}
