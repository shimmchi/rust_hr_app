use super::super::domain::person::id::PersonId;
use super::super::domain::person::list::PersonList;
use super::super::domain::person::Person;
use super::super::repository::person::PersonRepository;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct TextFilePerson {
    dir_path: String,
}

impl TextFilePerson {
    const FILE_NAME: &'static str = "/person.txt";
    pub fn new(path: &str) -> TextFilePerson {
        let dir_path = String::from(path);
        return TextFilePerson { dir_path: dir_path };
    }
}

impl PersonRepository for TextFilePerson {
    fn read_all_data(&self, person_list: &mut PersonList) -> Result<(), Box<dyn Error>> {
        let mut file_path = String::new();
        file_path.push_str(&self.dir_path[..]);
        file_path.push_str(&Self::FILE_NAME[..]);
        eprintln!("loading person file...");
        let mut file = match File::open(&file_path) {
            Ok(file) => file,
            Err(_) => {
                eprintln!("person file not found");
                return Ok(());
            }
        };
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let persons: Vec<&str> = contents.split_terminator('\n').collect();
        for person in persons {
            let info: Vec<&str> = person.split_whitespace().collect();
            if info.len() < 2 || info.len() > 3 {
                return Err("TextFilePerson::read_all_data : Invalid file format!".into());
            }
            let id: u64 = info[0].parse()?;
            let id = PersonId::new(id);
            let last_name = info[1];
            let first_name = if info.len() == 3 { Some(info[2]) } else { None };
            let new_person = Person::new(last_name, first_name)?;
            person_list.add_person(new_person, Some(id))?;
        }

        return Ok(());
    }

    fn overwrite_all_data(&self, person_list: &mut PersonList) -> Result<(), Box<dyn Error>> {
        let mut file_path = String::new();
        file_path.push_str(&self.dir_path[..]);
        file_path.push_str(&Self::FILE_NAME[..]);
        let mut file = match File::create(file_path) {
            Ok(file) => file,
            Err(e) => {
                return Err(e.into());
            }
        };

        for (id, name) in person_list.into_iter() {
            writeln!(file, "{} {}", id.to_string(), name)?;
        }

        return Ok(());
    }
}
