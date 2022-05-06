use super::super::domain::department::id::DepartmentId;
use super::super::domain::department::list::DepartmentList;
use super::super::domain::department::Department;
use super::super::repository::department::DepartmentRepository;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct TextFileDepartment {
    dir_path: String,
}

impl TextFileDepartment {
    const FILE_NAME: &'static str = "/departmetn.txt";
    pub fn new(path: &str) -> TextFileDepartment {
        let dir_path = String::from(path);
        return TextFileDepartment { dir_path: dir_path };
    }
}

impl DepartmentRepository for TextFileDepartment {
    fn read_all_data(&self, department_list: &mut DepartmentList) -> Result<(), Box<dyn Error>> {
        let mut file_path = String::new();
        file_path.push_str(&self.dir_path[..]);
        file_path.push_str(&Self::FILE_NAME[..]);
        eprintln!("loading department file...");
        let mut file = match File::open(&file_path) {
            Ok(file) => file,
            Err(_) => {
                eprintln!("department file not found");
                return Ok(());
            }
        };
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let departments: Vec<&str> = contents.split_terminator('\n').collect();
        for department in departments {
            let info: Vec<&str> = department.split_whitespace().collect();
            if info.len() != 2 {
                return Err("TextFileDepartment::read_all_data : Invalid file format!".into());
            }
            let id: u64 = info[0].parse()?;
            let id = DepartmentId::new(id);
            let department_name = info[1];
            let new_department = Department::new(department_name)?;
            department_list.add_department(new_department, Some(id))?;
        }

        return Ok(());
    }

    fn overwrite_all_data(
        &self,
        department_list: &mut DepartmentList,
    ) -> Result<(), Box<dyn Error>> {
        let mut file_path = String::new();
        file_path.push_str(&self.dir_path[..]);
        file_path.push_str(&Self::FILE_NAME[..]);
        let mut file = match File::create(file_path) {
            Ok(file) => file,
            Err(e) => {
                return Err(e.into());
            }
        };

        for (id, name) in department_list.into_iter() {
            writeln!(file, "{} {}", id.to_string(), name)?;
        }

        return Ok(());
    }
}
