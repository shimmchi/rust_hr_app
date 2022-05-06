use super::super::domain::department::id::DepartmentId;
use super::super::domain::department::list::DepartmentList;
use super::super::domain::department::Department;
use super::super::domain::member::MemberList;
use super::super::domain::person::id::PersonId;
use super::super::domain::person::list::PersonList;
use super::super::domain::person::Person;
use super::super::repository::department::DepartmentRepository;
use super::super::repository::member::MemberRepository;
use super::super::repository::person::PersonRepository;
use super::query::Query;
use std::collections::HashMap;
use std::error::Error;
use std::io;

// TODO
// テスト
// 大規模データ作成　性能テスト

pub fn run<P, D, M>(
    person_ripository: &P,
    department_repository: &D,
    member_repository: &M,
) -> Result<(), Box<dyn Error>>
where
    P: PersonRepository,
    D: DepartmentRepository,
    M: MemberRepository,
{
    let mut person_list = PersonList::new();
    match person_ripository.read_all_data(&mut person_list) {
        Ok(_) => (),
        Err(_) => panic!("person file: Invalid file format"),
    }

    let mut department_list = DepartmentList::new();
    match department_repository.read_all_data(&mut department_list) {
        Ok(_) => (),
        Err(_) => panic!("department file: Invalid file format"),
    }

    let mut member_list = MemberList::new();
    match member_repository.read_all_data(&mut member_list, &mut person_list, &mut department_list)
    {
        Ok(_) => (),
        Err(_) => panic!("member file: Invalid file format"),
    }

    println!("input a command");
    println!("if you need some help, input 'help'");

    loop {
        let mut command = String::new();

        io::stdin().read_line(&mut command)?;
        let command = command.trim().to_string();

        let query = Query::new(command);
        let query = match query {
            Ok(q) => q,
            Err(e) => {
                eprintln!("query error: {}", e);
                continue;
            }
        };
        println!();

        match query {
            Query::CreateDepartment { department_name } => {
                let new_department: Department = Department::new(&department_name)?;
                department_list.add_department(new_department, None)?;
                department_repository.overwrite_all_data(&mut department_list)?;
            }
            Query::ShowAllDepartments => {
                let list = &mut department_list;
                for (id, department_name) in list.into_iter() {
                    println!("{}: {}", id.to_string(), department_name);
                }
            }
            Query::ShowDepartmentInfo { department_id } => {
                let department_id: u64 = department_id.parse()?;
                let department_id = DepartmentId::new(department_id);
                let department = department_list.department(department_id)?;
                let department_name = department.name();
                let person_list =
                    member_list.person_list_by_department(department_id, &person_list)?;
                println!("{}:", department_name);
                for person in person_list.iter() {
                    println!(" {}", person.name());
                }
            }
            Query::DeleteDepartment { department_id } => {
                let id: u64 = department_id.parse()?;
                let id = DepartmentId::new(id);
                department_list.delete_department(id)?;
                department_repository.overwrite_all_data(&mut department_list)?;
            }
            Query::CreatePerson {
                last_name,
                first_name,
            } => {
                let new_person = match first_name {
                    Some(name) => {
                        let first_name = name;
                        Person::new(&last_name, Some(&first_name))?
                    }
                    None => Person::new(&last_name, None)?,
                };
                person_list.add_person(new_person, None)?;
                person_ripository.overwrite_all_data(&mut person_list)?;
            }
            Query::ShowAllPersons => {
                let list = &mut person_list;
                for (id, person_name) in list.into_iter() {
                    println!("{}: {}", id.to_string(), person_name);
                }
            }
            Query::ShowPersonInfo { person_id } => {
                let person_id: u64 = person_id.parse()?;
                let person_id = PersonId::new(person_id);
                let person = person_list.person(person_id)?;
                let person_name = person.name();
                let list =
                    member_list.department_list_by_person(person_id, &mut department_list)?;

                println!("{}:", person_name);
                for department in list.iter() {
                    println!(" {}", department.name());
                }
            }
            Query::DeletePerson { person_id } => {
                let id: u64 = person_id.parse()?;
                let id = PersonId::new(id);
                person_list.delete_person(id)?;
                person_ripository.overwrite_all_data(&mut person_list)?;
            }
            Query::AddPersonToDepartment {
                person_id,
                department_id,
            } => {
                let person_id: u64 = person_id.parse()?;
                let person_id = PersonId::new(person_id);
                let department_id: u64 = department_id.parse()?;
                let department_id = DepartmentId::new(department_id);
                member_list.add_person_to_department(
                    person_id,
                    department_id,
                    &person_list,
                    &department_list,
                )?;
                member_repository.overwrite_all_data(&mut member_list)?;
            }
            Query::RemovePersonFromDepartment {
                person_id,
                department_id,
            } => {
                let person_id: u64 = person_id.parse()?;
                let person_id = PersonId::new(person_id);
                let department_id: u64 = department_id.parse()?;
                let department_id = DepartmentId::new(department_id);
                member_list.remove_person_from_department(
                    person_id,
                    department_id,
                    &person_list,
                    &department_list,
                )?;
                member_repository.overwrite_all_data(&mut member_list)?;
            }
            Query::ShowAllPersonsByDepartment => {
                let mut department_member_list: HashMap<String, Vec<String>> = HashMap::new();
                let mut non_member_list: Vec<String> = Vec::new();
                for (_id, department_name) in department_list.into_iter() {
                    department_member_list.insert(department_name, Vec::new());
                }
                for (id, person_name) in person_list.into_iter() {
                    let list = member_list.department_list_by_person(id, &mut department_list)?;
                    if list.len() == 0 {
                        non_member_list.push(person_name);
                    } else {
                        for department in list.iter() {
                            let department_name = department.name();
                            let member_name_list =
                                match department_member_list.get_mut(&department_name) {
                                    Some(list) => list,
                                    None => {
                                        let message =
                                        "Query::ShowAllPersonsByDepartment : Department not found";
                                        return Err(message.into());
                                    }
                                };
                            member_name_list.push(person_name.clone());
                        }
                    }
                }
                for (department_name, person_list) in department_member_list.iter() {
                    println!("{}:", department_name);
                    for person in person_list.iter() {
                        println!(" {}", person);
                    }
                }
                println!("Not member of any department:");
                for person in non_member_list.iter() {
                    println!(" {}", person);
                }
            }
            Query::Help => {
                Query::print_help();
            }
            Query::Quit => {
                break;
            }
        }
    }
    return Ok(());
}
