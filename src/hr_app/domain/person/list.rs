use super::id::PersonId;
use super::*;
use std::collections::HashMap;
use std::error::Error;

// Personの一覧を格納する
// 一覧はPersonIdをキー、Option<Person>を値に持つHashMapとする
// max_idはHashMap内にある最も値の大きいIDを保持する
// max_idは新規にPersonを追加する際のIDの割り振りなどに使用する
// iter_idはIterator実装のためのメンバー変数
pub struct PersonList {
    max_id: PersonId,
    iter_id: PersonId,
    person_list: HashMap<PersonId, Option<Person>>,
}

impl PersonList {
    // ID=0はあらかじめNoneで埋めておく
    pub fn new() -> PersonList {
        let max_id = PersonId::new(0);
        let iter_id = PersonId::new(0);
        let mut list: HashMap<PersonId, Option<Person>> = HashMap::new();
        list.insert(max_id, None);
        return PersonList {
            max_id: max_id,
            iter_id: iter_id,
            person_list: list,
        };
    }

    // Personを渡してリストに追加する
    // 必要であればIDを指定することが出来る(プログラム起動時のファイル読み込みで使用)
    // 既にリストに存在しているIDと同じIDを重複して指定することは出来ない
    // 追加されたPersonのIDに応じてmax_idを更新する
    pub fn add_person(
        &mut self,
        person: Person,
        id: Option<PersonId>,
    ) -> Result<(), Box<dyn Error>> {
        match id {
            Some(id) => {
                match self.person_list.get(&id) {
                    Some(_) => {
                        let message = "PersonList::add_person : The same id has already existed";
                        return Err(message.into());
                    }
                    None => (),
                }
                self.person_list.insert(id, Some(person));
                if id.greater_than(self.max_id) {
                    self.max_id = id;
                }
            }
            None => {
                let next_id = self.max_id.next_id();
                self.person_list.insert(next_id, Some(person));
                self.max_id = next_id;
            }
        }
        return Ok(());
    }

    // IDを指定してPersonを削除する
    // HashMapのキーとなるIDは残し、値であるOption<Person>をNoneに変更する
    pub fn delete_person(&mut self, person_id: PersonId) -> Result<(), &'static str> {
        match self.person_list.get(&person_id) {
            Some(option) => match option {
                Some(_) => {
                    self.person_list.insert(person_id, None);
                    return Ok(());
                }
                None => {
                    return Err(
                        "PersonList::delete_person : This person has already been deleted!",
                    );
                }
            },
            None => {
                return Err("PersonList::delete_person : Id is empty!");
            }
        }
    }

    // IDを指定してPersonの参照を取得する
    // 存在しないIDや削除済のIDを指定するとエラーを返す
    pub fn person(&self, person_id: PersonId) -> Result<&Person, &'static str> {
        let person = match self.person_list.get(&person_id) {
            Some(option) => match option {
                Some(person) => person,
                None => {
                    let message = "PersonList::person : This person has already been deleted!";
                    return Err(message);
                }
            },
            None => {
                let message = "PersonList::person : Id is empty!";
                return Err(message);
            }
        };
        return Ok(person);
    }
}

// IDの小さい順にPersonIdと対応するPersonの名前を文字列として返す
// 存在しないIDや削除済のPersonは無視される
impl Iterator for &mut PersonList {
    type Item = (PersonId, String);
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.iter_id = self.iter_id.next_id();
            if self.iter_id.greater_than(self.max_id) {
                self.iter_id = PersonId::new(0);
                return None;
            }
            match self.person(self.iter_id) {
                Ok(person) => {
                    return Some((self.iter_id, person.name()));
                }
                Err(_) => (),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_a_person() -> Person {
        let person = Person::new("Sample", None).unwrap();
        return person;
    }

    fn extract_id(item: Option<(PersonId, String)>) -> Option<PersonId> {
        match item {
            Some((id, _name)) => return Some(id),
            None => return None,
        }
    }

    #[test]
    fn add_person_no_id_ok() {
        let mut person_list = PersonList::new();
        let person1 = generate_a_person();
        let person2 = generate_a_person();
        assert!(person_list.add_person(person1, None).is_ok());
        assert!(person_list.add_person(person2, None).is_ok());
    }

    #[test]
    fn add_person_with_id_ok() {
        let mut person_list = PersonList::new();
        let person1 = generate_a_person();
        let person2 = generate_a_person();
        assert!(person_list
            .add_person(person1, Some(PersonId::new(1)))
            .is_ok());
        assert!(person_list
            .add_person(person2, Some(PersonId::new(2)))
            .is_ok());
    }

    #[test]
    fn add_person_with_same_id_err() {
        let mut person_list = PersonList::new();
        let person1 = generate_a_person();
        let person2 = generate_a_person();
        assert!(person_list
            .add_person(person1, Some(PersonId::new(1)))
            .is_ok());
        assert!(person_list
            .add_person(person2, Some(PersonId::new(1)))
            .is_err());
    }

    #[test]
    fn delete_person_ok() {
        let mut person_list = PersonList::new();
        let person = generate_a_person();
        person_list
            .add_person(person, Some(PersonId::new(1)))
            .unwrap();
        assert!(person_list.delete_person(PersonId::new(1)).is_ok());
    }

    #[test]
    fn delete_person_empty_id_err() {
        let mut person_list = PersonList::new();
        assert!(person_list.delete_person(PersonId::new(1)).is_err());
    }

    #[test]
    fn delete_person_same_person_err() {
        let mut person_list = PersonList::new();
        let person = generate_a_person();
        let id = PersonId::new(1);
        person_list.add_person(person, Some(id)).unwrap();
        assert!(person_list.delete_person(id).is_ok());
        assert!(person_list.delete_person(id).is_err());
    }

    #[test]
    fn person_ok() {
        let mut person_list = PersonList::new();
        let person = generate_a_person();
        let id = PersonId::new(1);
        person_list.add_person(person, Some(id)).unwrap();
        assert!(person_list.person(id).is_ok());
    }

    #[test]
    fn person_empty_id_err() {
        let person_list = PersonList::new();
        assert!(person_list.person(PersonId::new(1)).is_err());
    }

    #[test]
    fn person_deleted_person_err() {
        let mut person_list = PersonList::new();
        let person = generate_a_person();
        let id = PersonId::new(1);
        person_list.add_person(person, Some(id)).unwrap();
        assert!(person_list.delete_person(id).is_ok());
        assert!(person_list.delete_person(id).is_err());
    }

    #[test]
    fn iterator_ok_1() {
        let mut person_list = PersonList::new();
        let person1 = generate_a_person();
        let person2 = generate_a_person();
        let person3 = generate_a_person();

        person_list.add_person(person1, None).unwrap();
        person_list.add_person(person2, None).unwrap();
        person_list.add_person(person3, None).unwrap();

        let mut iter = person_list.into_iter();
        assert_eq!(extract_id(iter.next()), Some(PersonId::new(1)));
        assert_eq!(extract_id(iter.next()), Some(PersonId::new(2)));
        assert_eq!(extract_id(iter.next()), Some(PersonId::new(3)));
        assert_eq!(extract_id(iter.next()), None);
    }

    #[test]
    fn iterator_ok_2() {
        let mut person_list = PersonList::new();
        let person1 = generate_a_person();
        let person2 = generate_a_person();
        let person3 = generate_a_person();

        person_list
            .add_person(person1, Some(PersonId::new(2)))
            .unwrap();
        person_list
            .add_person(person2, Some(PersonId::new(5)))
            .unwrap();
        person_list.delete_person(PersonId::new(5)).unwrap();
        person_list.add_person(person3, None).unwrap();

        let mut iter = person_list.into_iter();
        assert_eq!(extract_id(iter.next()), Some(PersonId::new(2)));
        assert_eq!(extract_id(iter.next()), Some(PersonId::new(6)));
        assert_eq!(extract_id(iter.next()), None);
    }
}
