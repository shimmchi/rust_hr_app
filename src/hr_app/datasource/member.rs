use super::super::domain::department::id::DepartmentId;
use super::super::domain::department::list::DepartmentList;
use super::super::domain::member::MemberList;
use super::super::domain::person::id::PersonId;
use super::super::domain::person::list::PersonList;
use super::super::repository::member::MemberRepository;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct TextFileMember {
    dir_path: String,
}

impl TextFileMember {
    const FILE_NAME: &'static str = "/member.txt";
    pub fn new(path: &str) -> TextFileMember {
        let dir_path = String::from(path);
        return TextFileMember { dir_path: dir_path };
    }
}

impl MemberRepository for TextFileMember {
    fn read_all_data(
        &self,
        member_list: &mut MemberList,
        person_list: &PersonList,
        department_list: &DepartmentList,
    ) -> Result<(), Box<dyn Error>> {
        let mut file_path = String::new();
        file_path.push_str(&self.dir_path[..]);
        file_path.push_str(&Self::FILE_NAME[..]);
        eprintln!("loading member file...");
        let mut file = match File::open(&file_path) {
            Ok(file) => file,
            Err(_) => {
                eprintln!("member file not found");
                return Ok(());
            }
        };
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let members: Vec<&str> = contents.split_terminator('\n').collect();
        for member in members {
            let info: Vec<&str> = member.split_whitespace().collect();
            if info.len() != 2 {
                return Err("TextFileMember::read_all_data : Invalid file format!".into());
            }
            let department_id: u64 = info[0].parse()?;
            let department_id = DepartmentId::new(department_id);
            let person_id: u64 = info[1].parse()?;
            let person_id = PersonId::new(person_id);
            member_list.add_person_to_department(
                person_id,
                department_id,
                person_list,
                department_list,
            )?;
        }

        return Ok(());
    }

    fn overwrite_all_data(&self, member_list: &mut MemberList) -> Result<(), Box<dyn Error>> {
        let mut file_path = String::new();
        file_path.push_str(&self.dir_path[..]);
        file_path.push_str(&Self::FILE_NAME[..]);
        let mut file = match File::create(file_path) {
            Ok(file) => file,
            Err(e) => {
                return Err(e.into());
            }
        };

        for (department_id, person_id_list) in member_list.into_iter() {
            for person_id in person_id_list {
                writeln!(
                    file,
                    "{} {}",
                    department_id.to_string(),
                    person_id.to_string()
                )?;
            }
        }

        return Ok(());
    }
}
