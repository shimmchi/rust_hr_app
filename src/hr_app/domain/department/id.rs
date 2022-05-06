use super::super::id_type::Id;

// Department用のIDを格納する
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct DepartmentId {
    value: Id,
}

// メソッドはIdを同じ
impl DepartmentId {
    pub fn new(value: u64) -> DepartmentId {
        let value = Id::new(value);
        return DepartmentId { value: value };
    }

    pub fn next_id(&self) -> DepartmentId {
        let next_value = self.value.next_id();
        return DepartmentId { value: next_value };
    }

    fn value(&self) -> Id {
        return self.value;
    }

    pub fn greater_than(&self, another_id: DepartmentId) -> bool {
        return self.value.greater_than(another_id.value());
    }

    pub fn to_string(&self) -> String {
        return self.value.to_string();
    }
}
