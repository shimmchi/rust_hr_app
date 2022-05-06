use super::super::id_type::Id;

// Person用のIDを格納する
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct PersonId {
    value: Id,
}

// メソッドはIdと同じ
impl PersonId {
    pub fn new(value: u64) -> PersonId {
        let value = Id::new(value);
        return PersonId { value: value };
    }

    pub fn next_id(&self) -> PersonId {
        let next_value = self.value.next_id();
        return PersonId { value: next_value };
    }

    fn value(&self) -> Id {
        return self.value;
    }

    pub fn greater_than(&self, another_id: PersonId) -> bool {
        return self.value.greater_than(another_id.value());
    }

    pub fn to_string(&self) -> String {
        return self.value.to_string();
    }
}
