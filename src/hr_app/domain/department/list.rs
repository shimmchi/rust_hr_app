use super::id::DepartmentId;
use super::*;
use std::collections::HashMap;

// Departmentの一覧を格納する
// 一覧はDepartmentIdをキー、Option<Department>を値に持つHashMapとする
// max_idはHashMap内にある最も値の大きいIDを保持する
// max_idは新規にDepartmentを追加する際のIDの割り振りなどに使用する
// iter_idはIterator実装のためのメンバー変数
pub struct DepartmentList {
    max_id: DepartmentId,
    iter_id: DepartmentId,
    department_list: HashMap<DepartmentId, Option<Department>>,
}

impl DepartmentList {
    // ID=0はあらかじめNoneで埋めておく
    pub fn new() -> DepartmentList {
        let max_id = DepartmentId::new(0);
        let iter_id = DepartmentId::new(0);
        let mut list: HashMap<DepartmentId, Option<Department>> = HashMap::new();
        list.insert(max_id, None);
        return DepartmentList {
            max_id: max_id,
            iter_id: iter_id,
            department_list: list,
        };
    }

    // Departmentを渡してリストに追加する
    // 必要であればIDを指定することが出来る(プログラム起動時のファイル読み込みで使用)
    // 同名の部署は複数存在できない
    // 既にリストに存在しているIDと同じIDを重複して指定することは出来ない
    // 追加されたDepartmentのIDに応じてmax_idを更新する
    pub fn add_department(
        &mut self,
        department: Department,
        id: Option<DepartmentId>,
    ) -> Result<(), &'static str> {
        let new_department_name = department.name();
        for (_id, old_department_name) in self.into_iter() {
            if new_department_name == old_department_name {
                let message = "DepartmetList::add_department : The same name already exists!";
                return Err(message);
            }
        }

        match id {
            Some(id) => {
                match self.department_list.get(&id) {
                    Some(_) => {
                        let message =
                            "DepartmentList::add_department : The same id has already existed";
                        return Err(message.into());
                    }
                    None => (),
                }
                self.department_list.insert(id, Some(department));
                if id.greater_than(self.max_id) {
                    self.max_id = id;
                }
            }
            None => {
                let next_id = self.max_id.next_id();
                self.department_list.insert(next_id, Some(department));
                self.max_id = next_id;
            }
        }

        return Ok(());
    }

    // IDを指定してDepartmentを削除する
    // HashMapのキーとなるIDは残し、値であるOption<Department>をNoneに変更する
    pub fn delete_department(&mut self, department_id: DepartmentId) -> Result<(), &'static str> {
        match self.department_list.get(&department_id) {
            Some(option) => match option {
                Some(_) => {
                    self.department_list.insert(department_id, None);
                    return Ok(());
                }
                None => {
                    return Err("DepartmentList::delete_department : This department has already been deleted!");
                }
            },
            None => {
                return Err("DepartmentList::delete_department : Received id is empty!");
            }
        }
    }

    // IDを指定してDepartmentの参照を取得する
    // 存在しないIDや削除済のIDを指定するとエラーを返す
    pub fn department(&self, department_id: DepartmentId) -> Result<&Department, &'static str> {
        let department = match self.department_list.get(&department_id) {
            Some(option) => match option {
                Some(department) => department,
                None => {
                    let message =
                        "DepartmentList::department : This department has already been deleted!";
                    return Err(message);
                }
            },
            None => {
                let message = "DepartmentLsit:department : This id is empty!";
                return Err(message);
            }
        };
        return Ok(department);
    }
}

// IDの小さい順にDepartmentIdと対応するDepartmentの名前を文字列として返す
// 存在しないIDや削除済のDepartmentは無視される
impl Iterator for &mut DepartmentList {
    type Item = (DepartmentId, String);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.iter_id = self.iter_id.next_id();
            if self.iter_id.greater_than(self.max_id) {
                self.iter_id = DepartmentId::new(0);
                return None;
            }
            match self.department(self.iter_id) {
                Ok(department) => {
                    return Some((self.iter_id, department.name()));
                }
                Err(_) => (),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_a_department(s: &str) -> Department {
        let mut name = String::from("Sample");
        name.push_str(s);
        let department = Department::new(&name).unwrap();
        return department;
    }

    fn extract_id(item: Option<(DepartmentId, String)>) -> Option<DepartmentId> {
        match item {
            Some((id, _name)) => return Some(id),
            None => return None,
        }
    }

    #[test]
    fn add_department_no_id_ok() {
        let mut department_list = DepartmentList::new();
        let department1 = generate_a_department("1");
        let department2 = generate_a_department("2");
        assert!(department_list.add_department(department1, None).is_ok());
        assert!(department_list.add_department(department2, None).is_ok());
    }

    #[test]
    fn add_department_with_id_ok() {
        let mut department_list = DepartmentList::new();
        let department1 = generate_a_department("1");
        let department2 = generate_a_department("2");
        assert!(department_list
            .add_department(department1, Some(DepartmentId::new(1)))
            .is_ok());
        assert!(department_list
            .add_department(department2, Some(DepartmentId::new(2)))
            .is_ok());
    }

    #[test]
    fn add_department_with_same_id_err() {
        let mut department_list = DepartmentList::new();
        let department1 = generate_a_department("1");
        let department2 = generate_a_department("2");
        assert!(department_list
            .add_department(department1, Some(DepartmentId::new(1)))
            .is_ok());
        assert!(department_list
            .add_department(department2, Some(DepartmentId::new(1)))
            .is_err());
    }

    #[test]
    fn add_department_same_name_err() {
        let mut department_list = DepartmentList::new();
        let department1 = generate_a_department("1");
        let department2 = generate_a_department("1");
        assert!(department_list
            .add_department(department1, Some(DepartmentId::new(1)))
            .is_ok());
        assert!(department_list
            .add_department(department2, Some(DepartmentId::new(2)))
            .is_err());
    }

    #[test]
    fn delete_department_ok() {
        let mut department_list = DepartmentList::new();
        let department = generate_a_department("1");
        department_list
            .add_department(department, Some(DepartmentId::new(1)))
            .unwrap();
        assert!(department_list
            .delete_department(DepartmentId::new(1))
            .is_ok());
    }

    #[test]
    fn delete_department_empty_id_err() {
        let mut department_list = DepartmentList::new();
        assert!(department_list
            .delete_department(DepartmentId::new(1))
            .is_err());
    }

    #[test]
    fn delete_department_same_person_err() {
        let mut department_list = DepartmentList::new();
        let department = generate_a_department("1");
        let id = DepartmentId::new(1);
        department_list
            .add_department(department, Some(id))
            .unwrap();
        assert!(department_list.delete_department(id).is_ok());
        assert!(department_list.delete_department(id).is_err());
    }

    #[test]
    fn department_ok() {
        let mut department_list = DepartmentList::new();
        let department = generate_a_department("1");
        let id = DepartmentId::new(1);
        department_list
            .add_department(department, Some(id))
            .unwrap();
        assert!(department_list.department(id).is_ok());
    }

    #[test]
    fn department_empty_id_err() {
        let department_list = DepartmentList::new();
        assert!(department_list.department(DepartmentId::new(1)).is_err());
    }

    #[test]
    fn department_deleted_person_err() {
        let mut department_list = DepartmentList::new();
        let department = generate_a_department("1");
        let id = DepartmentId::new(1);
        department_list
            .add_department(department, Some(id))
            .unwrap();
        assert!(department_list.delete_department(id).is_ok());
        assert!(department_list.delete_department(id).is_err());
    }

    #[test]
    fn iterator_ok_1() {
        let mut department_list = DepartmentList::new();
        let department1 = generate_a_department("1");
        let department2 = generate_a_department("2");
        let department3 = generate_a_department("3");

        department_list.add_department(department1, None).unwrap();
        department_list.add_department(department2, None).unwrap();
        department_list.add_department(department3, None).unwrap();

        let mut iter = department_list.into_iter();
        assert_eq!(extract_id(iter.next()), Some(DepartmentId::new(1)));
        assert_eq!(extract_id(iter.next()), Some(DepartmentId::new(2)));
        assert_eq!(extract_id(iter.next()), Some(DepartmentId::new(3)));
        assert_eq!(extract_id(iter.next()), None);
    }

    #[test]
    fn iterator_ok_2() {
        let mut department_list = DepartmentList::new();
        let department1 = generate_a_department("1");
        let department2 = generate_a_department("2");
        let department3 = generate_a_department("3");

        department_list
            .add_department(department1, Some(DepartmentId::new(2)))
            .unwrap();
        department_list
            .add_department(department2, Some(DepartmentId::new(5)))
            .unwrap();
        department_list
            .delete_department(DepartmentId::new(5))
            .unwrap();
        department_list.add_department(department3, None).unwrap();

        let mut iter = department_list.into_iter();
        assert_eq!(extract_id(iter.next()), Some(DepartmentId::new(2)));
        assert_eq!(extract_id(iter.next()), Some(DepartmentId::new(6)));
        assert_eq!(extract_id(iter.next()), None);
    }
}
