use super::department::id::DepartmentId;
use super::department::list::DepartmentList;
use super::department::*;
use super::person::id::PersonId;
use super::person::list::PersonList;
use super::person::*;
use std::collections::HashMap;

// HashMapの値を埋めるために定義したが、プログラム内で使用しない
enum MemberStatus {
    Valid,
}

// 個人と部署の紐付けの一覧を格納する
//　DepartmentIdをキーとしたHashMap
// 上記HashMapの値はPersonIdをキーとしたHashMapとなっている。このHashMapの値は使用しない
// 部署Aに個人Sが所属している時、キーAに対応するHashMapにキーSが存在する
// 部署Aに所属する個人一覧はO(1)で取り出せる
// 部署Aに個人Sが所属しているかどうかもO(1)で取り出せる
// 個人Sが所属している部署一覧の取得はO(n) (全ての部署を調べる必要がある)
// iter_id, max_idはIterator実装のためのメンバー変数
pub struct MemberList {
    iter_id: DepartmentId,
    max_id: DepartmentId,
    list: HashMap<DepartmentId, HashMap<PersonId, MemberStatus>>,
}

impl MemberList {
    pub fn new() -> MemberList {
        let iter_id = DepartmentId::new(0);
        let max_id = DepartmentId::new(0);
        let list: HashMap<DepartmentId, HashMap<PersonId, MemberStatus>> = HashMap::new();
        return MemberList {
            iter_id: iter_id,
            max_id: max_id,
            list: list,
        };
    }

    // 個人と部署の紐付けを追加する
    // DepartmentIdがlistに存在しない場合はキーを追加する
    // 追加する部署のDepartmentIdに対応するHashMapに追加する個人のPersonIdをキーとして追加する
    // １つの部署に同じ人エンティティは１つしか所属できない
    // PersonListに存在しない個人を部署に追加することはできない
    // DepartmentListに存在しない部署に個人を追加することはできない
    pub fn add_person_to_department(
        &mut self,
        person_id: PersonId,
        department_id: DepartmentId,
        person_list: &PersonList,
        department_list: &DepartmentList,
    ) -> Result<(), &'static str> {
        let _person = person_list.person(person_id)?;
        let _department = department_list.department(department_id)?;

        let member_map_of_the_department = self.list.entry(department_id).or_insert(HashMap::new());
        if department_id.greater_than(self.max_id) {
            self.max_id = department_id;
        }

        match member_map_of_the_department.get(&person_id) {
            Some(_) => {
                let message = "MemberList::add_person_to_department : This person has already belonged to the department!";
                return Err(message);
            }
            None => {
                member_map_of_the_department.insert(person_id, MemberStatus::Valid);
            }
        }
        return Ok(());
    }

    // 部署と個人の紐付けを削除する
    // 部署に所属する人HashMapから削除対象の個人のPersonIdキーを削除する
    // 部署に所属していない個人と部署との紐付けを解除することは出来ない
    // PersonListに存在しない個人を削除することは出来ない
    // DepartmentListに存在しない部署への所属を削除することはできない
    pub fn remove_person_from_department(
        &mut self,
        person_id: PersonId,
        department_id: DepartmentId,
        person_list: &PersonList,
        department_list: &DepartmentList,
    ) -> Result<(), &'static str> {
        let _person = person_list.person(person_id)?;
        let _department = department_list.department(department_id)?;

        let member_map_of_the_department = match self.list.get_mut(&department_id) {
            Some(list) => list,
            None => {
                let message = "MemberList::remove_person_from_department : DepartmentId is empty!";
                return Err(message);
            }
        };
        match member_map_of_the_department.get(&person_id) {
            Some(_) => (),
            None => {
                let message =
                    "MemberList::remove_person_from_department : This person is not a member!";
                return Err(message);
            }
        }
        member_map_of_the_department.remove(&person_id);

        return Ok(());
    }

    // 部署を指定して、その部署に所属している個人の一覧を取得する
    // 指定された部署がMemberListに存在しない場合は空のリストが返される
    // ※HashMapのイテレータは毎回順序が変わるので、返される個人リストの順番も実行ごとに変わる
    pub fn person_list_by_department<'a>(
        &self,
        department_id: DepartmentId,
        person_list: &'a PersonList,
    ) -> Result<Vec<&'a Person>, &'static str> {
        let mut result_person_list: Vec<&Person> = Vec::new();
        let member_map_of_the_department = match self.list.get(&department_id) {
            Some(list) => list,
            None => {
                return Ok(result_person_list);
            }
        };

        for (id, _status) in member_map_of_the_department.iter() {
            let person: &Person = match person_list.person(*id) {
                Ok(person) => person,
                Err(_) => continue,
            };
            result_person_list.push(person);
        }
        return Ok(result_person_list);
    }

    // 個人を指定して、その個人が所属している部署の一覧を取得する
    // 指定された個人がMemberListに存在しない場合は空のリストが返される
    pub fn department_list_by_person<'a>(
        &self,
        person_id: PersonId,
        department_list: &'a mut DepartmentList,
    ) -> Result<Vec<&'a Department>, &'static str> {
        let mut result_department_list: Vec<&Department> = Vec::new();
        for (department_id, member_map_of_the_department) in self.list.iter() {
            if member_map_of_the_department.contains_key(&person_id) {
                let department: &Department = department_list.department(*department_id)?;
                result_department_list.push(department);
            }
        }
        return Ok(result_department_list);
    }
}

impl Iterator for &mut MemberList {
    type Item = (DepartmentId, Vec<PersonId>);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.iter_id = self.iter_id.next_id();
            if self.iter_id.greater_than(self.max_id) {
                self.iter_id = DepartmentId::new(0);
                return None;
            }
            match self.list.get(&self.iter_id) {
                Some(list) => {
                    let mut person_id_list: Vec<PersonId> = Vec::new();
                    for (id, _status) in list {
                        person_id_list.push(*id);
                    }
                    return Some((self.iter_id, person_id_list));
                }
                None => (),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    fn generate_a_person() -> Person {
        let person = Person::new("Sample", None).unwrap();
        return person;
    }

    fn generate_a_department(s: &str) -> Department {
        let mut name = String::from("Sample");
        name.push_str(s);
        let department = Department::new(&name).unwrap();
        return department;
    }

    struct Lists {
        person_list: PersonList,
        department_list: DepartmentList,
        member_list: MemberList,
    }

    impl Lists {
        fn new() -> Lists {
            let person_list = PersonList::new();
            let department_list = DepartmentList::new();
            let member_list = MemberList::new();
            return Lists {
                person_list: person_list,
                department_list: department_list,
                member_list: member_list,
            };
        }

        fn create_person(&mut self, num: i8) {
            for _ in 0..num {
                let person = generate_a_person();
                self.person_list.add_person(person, None).unwrap();
            }
        }

        fn create_department(&mut self, num: i8) {
            for i in 0..num {
                let department = generate_a_department(&i.to_string());
                self.department_list
                    .add_department(department, None)
                    .unwrap();
            }
        }

        fn add_member(
            &mut self,
            person_id: PersonId,
            department_id: DepartmentId,
        ) -> Result<(), Box<dyn Error>> {
            self.member_list.add_person_to_department(
                person_id,
                department_id,
                &self.person_list,
                &self.department_list,
            )?;
            return Ok(());
        }

        fn remove_member(
            &mut self,
            person_id: PersonId,
            department_id: DepartmentId,
        ) -> Result<(), Box<dyn Error>> {
            self.member_list.remove_person_from_department(
                person_id,
                department_id,
                &self.person_list,
                &self.department_list,
            )?;
            return Ok(());
        }
    }

    #[test]
    fn add_person_to_department_ok() {
        let mut test_list = Lists::new();
        test_list.create_person(2);
        test_list.create_department(2);

        assert!(test_list
            .add_member(PersonId::new(1), DepartmentId::new(1))
            .is_ok());

        assert!(test_list
            .add_member(PersonId::new(2), DepartmentId::new(1))
            .is_ok());

        assert!(test_list
            .add_member(PersonId::new(1), DepartmentId::new(2))
            .is_ok());

        assert!(test_list
            .add_member(PersonId::new(2), DepartmentId::new(2))
            .is_ok());
    }

    #[test]
    fn add_person_to_department_same_person_err() {
        let mut test_list = Lists::new();
        test_list.create_person(1);
        test_list.create_department(1);

        assert!(test_list
            .add_member(PersonId::new(1), DepartmentId::new(1))
            .is_ok());

        assert!(test_list
            .add_member(PersonId::new(1), DepartmentId::new(1))
            .is_err());
    }

    #[test]
    fn add_person_to_department_person_not_exist_err() {
        let mut test_list = Lists::new();
        test_list.create_department(1);
        assert!(test_list
            .add_member(PersonId::new(1), DepartmentId::new(1))
            .is_err());
    }

    #[test]
    fn add_person_to_department_department_not_exist_err() {
        let mut test_list = Lists::new();
        test_list.create_person(1);
        assert!(test_list
            .add_member(PersonId::new(1), DepartmentId::new(1))
            .is_err());
    }

    #[test]
    fn remove_person_from_department_ok() {
        let mut test_list = Lists::new();
        test_list.create_person(2);
        test_list.create_department(2);
        test_list
            .add_member(PersonId::new(1), DepartmentId::new(1))
            .unwrap();
        test_list
            .add_member(PersonId::new(2), DepartmentId::new(1))
            .unwrap();
        test_list
            .add_member(PersonId::new(1), DepartmentId::new(2))
            .unwrap();
        test_list
            .add_member(PersonId::new(2), DepartmentId::new(2))
            .unwrap();

        assert!(test_list
            .remove_member(PersonId::new(1), DepartmentId::new(1))
            .is_ok());
        assert!(test_list
            .remove_member(PersonId::new(2), DepartmentId::new(1))
            .is_ok());
        assert!(test_list
            .remove_member(PersonId::new(1), DepartmentId::new(2))
            .is_ok());
        assert!(test_list
            .remove_member(PersonId::new(2), DepartmentId::new(2))
            .is_ok());
    }

    #[test]
    fn remove_person_from_department_same_person_err() {
        let mut test_list = Lists::new();
        test_list.create_person(1);
        test_list.create_department(1);
        test_list
            .add_member(PersonId::new(1), DepartmentId::new(1))
            .unwrap();

        assert!(test_list
            .remove_member(PersonId::new(1), DepartmentId::new(1))
            .is_ok());
        assert!(test_list
            .remove_member(PersonId::new(1), DepartmentId::new(1))
            .is_err());
    }

    #[test]
    fn remove_person_from_department_person_not_exist_err() {
        let mut test_list = Lists::new();
        test_list.create_person(1);
        test_list.create_department(1);
        test_list
            .add_member(PersonId::new(1), DepartmentId::new(1))
            .unwrap();

        assert!(test_list
            .remove_member(PersonId::new(2), DepartmentId::new(1))
            .is_err());
    }

    #[test]
    fn remove_person_from_department_department_not_exist_err() {
        let mut test_list = Lists::new();
        test_list.create_person(1);
        test_list.create_department(1);
        test_list
            .add_member(PersonId::new(1), DepartmentId::new(1))
            .unwrap();

        assert!(test_list
            .remove_member(PersonId::new(1), DepartmentId::new(2))
            .is_err());
    }
}
