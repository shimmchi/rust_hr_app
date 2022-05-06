// u64の整数値をラップする
// HashMapのキーとして使用する
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Id {
    value: u64,
}

impl Id {
    pub fn new(value: u64) -> Id {
        return Id { value: value };
    }

    // 値を1だけインクリメントしたIDを返す
    pub fn next_id(&self) -> Id {
        let next_value = self.value + 1;
        return Id { value: next_value };
    }

    fn value(&self) -> u64 {
        return self.value;
    }

    // IDが引数IDよりも大きい時にtrueを返す
    // ID同士の順序の比較に使用する
    pub fn greater_than(&self, another_id: Id) -> bool {
        if self.value > another_id.value() {
            return true;
        }
        return false;
    }

    // 画面出力用に文字列を返す
    pub fn to_string(&self) -> String {
        return self.value.to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_id_ok() {
        let id = Id::new(1).next_id();
        let ans = Id::new(2);
        assert_eq!(id, ans);
    }

    #[test]
    fn greater_than_true() {
        let small = Id::new(1);
        let large = Id::new(10);
        assert_eq!(large.greater_than(small), true);
    }

    #[test]
    fn greater_than_false() {
        let small = Id::new(1);
        let large = Id::new(10);
        assert_eq!(small.greater_than(large), false);
    }

    #[test]
    fn greater_than_eaual_false() {
        let id1 = Id::new(5);
        let id2 = Id::new(5);
        assert_eq!(id1.greater_than(id2), false);
    }

    #[test]
    fn to_string_ok() {
        let ans = String::from("100");
        let id = Id::new(100);
        assert_eq!(id.to_string(), ans);
    }
}
