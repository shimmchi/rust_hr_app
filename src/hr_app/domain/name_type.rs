// 最長値と最短値を持つ文字列を格納する
pub struct NameType {
    value: String,
    _min_length: u8,
    _max_length: u8,
}

impl NameType {
    // 最長値と最短値はともに正の値
    // 最長値は最短値よりも大きくなければならない
    // name引数の長さが最短値以上最長値以下に収まらなければエラーを返す
    pub fn new(name: &str, min_length: u8, max_length: u8) -> Result<NameType, &'static str> {
        let min_length = if min_length > 0 {
            min_length
        } else {
            return Err("NameType::new : min_length must be larger than 0.");
        };

        let max_length = if max_length > min_length {
            max_length
        } else {
            return Err("NameType::new : max_length must be larger than min_length.");
        };

        let name_length: usize = name.len();
        if name_length > max_length as usize {
            return Err("Name::new : the size of value is larger than the max length.");
        }
        if name_length < min_length as usize {
            return Err("Name::new : the size of value is smaller than the min length");
        }

        return Ok(NameType {
            value: String::from(name),
            _min_length: min_length,
            _max_length: max_length,
        });
    }

    pub fn value(&self) -> &str {
        return &self.value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_ok() {
        assert!(NameType::new("Bob", 1, 20).is_ok());
    }

    #[test]
    fn new_min_length_bounday_err() {
        assert!(NameType::new("Bob", 0, 20).is_err());
    }

    #[test]
    fn new_min_length_equal_to_max_length_err() {
        assert!(NameType::new("Bob", 20, 20).is_err());
    }

    #[test]
    fn new_max_length_smaller_than_min_length_err() {
        assert!(NameType::new("Bob", 20, 19).is_err());
    }

    #[test]
    fn new_name_length_equal_to_min_length_ok() {
        assert!(NameType::new("abcde", 5, 10).is_ok());
    }

    #[test]
    fn new_name_length_too_short_err() {
        assert!(NameType::new("abcd", 5, 10).is_err());
    }

    #[test]
    fn new_name_length_equal_to_max_length_ok() {
        assert!(NameType::new("abcdefghij", 5, 10).is_ok());
    }

    #[test]
    fn new_name_length_too_large_err() {
        assert!(NameType::new("abcdefghijk", 5, 10).is_err());
    }

    #[test]
    fn value_ok() {
        let name_type = NameType::new("Alice", 1, 20).unwrap();
        assert_eq!(name_type.value(), "Alice");
    }
}
