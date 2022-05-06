extern crate human_management;

use human_management::hr_app::application::service;
use human_management::hr_app::datasource::department::TextFileDepartment;
use human_management::hr_app::datasource::member::TextFileMember;
use human_management::hr_app::datasource::person::TextFilePerson;

fn main() {
    let dir_path = "./src/hr_app/datasource/text/";
    let person_datasouace = TextFilePerson::new(dir_path);
    let department_datasource = TextFileDepartment::new(dir_path);
    let member_datasource = TextFileMember::new(dir_path);
    loop {
        if let Err(e) = service::run(
            &person_datasouace,
            &department_datasource,
            &member_datasource,
        ) {
            eprintln!("Application Error: {}", e);
            continue;
        } else {
            break;
        }
    }
    println!("Application finished successfully!");
}
