pub enum Query {
    CreateDepartment {
        department_name: String,
    },

    ShowAllDepartments,

    // DONE
    ShowDepartmentInfo {
        department_id: String,
    },

    // DONE
    DeleteDepartment {
        department_id: String,
    },

    // Done
    CreatePerson {
        last_name: String,
        first_name: Option<String>,
    },

    // Done
    ShowAllPersons,

    // DONE
    ShowPersonInfo {
        person_id: String,
    },

    // DONE
    DeletePerson {
        person_id: String,
    },

    // DONE
    AddPersonToDepartment {
        person_id: String,
        department_id: String,
    },
    RemovePersonFromDepartment {
        person_id: String,
        department_id: String,
    },
    ShowAllPersonsByDepartment,
    Help,
    Quit,
}

impl Query {
    pub fn new(commands: String) -> Result<Query, String> {
        let mut command = commands.split_whitespace();

        match command.next() {
            Some("create_department") => {
                let name = match command.next() {
                    Some(name) => name,
                    None => {
                        let message = Query::err_message(
                            "department_name is missing",
                            "create_department <department_name>",
                        );
                        return Err(message);
                    }
                };
                return Ok(Query::CreateDepartment {
                    department_name: String::from(name),
                });
            }
            Some("show_all_departments") => return Ok(Query::ShowAllDepartments),
            Some("department_info") => {
                let id = match command.next() {
                    Some(id) => String::from(id),
                    None => {
                        let message = Query::err_message(
                            "department_id is missing",
                            "department_info <department_id>",
                        );
                        return Err(message);
                    }
                };
                return Ok(Query::ShowDepartmentInfo { department_id: id });
            }
            Some("delete_department") => {
                let id = match command.next() {
                    Some(id) => id,
                    None => {
                        let message = Query::err_message(
                            "department_id is missing",
                            "delete_department <department_id>",
                        );
                        return Err(message);
                    }
                };
                return Ok(Query::DeleteDepartment {
                    department_id: String::from(id),
                });
            }
            Some("create_person") => {
                let last_name = match command.next() {
                    Some(name) => name,
                    None => {
                        let message = Query::err_message(
                            "person_name is missing",
                            "create_person <last_name> <first_name>(optional)",
                        );
                        return Err(message);
                    }
                };
                let first_name: Option<String> = match command.next() {
                    Some(name) => Some(String::from(name)),
                    None => None,
                };

                return Ok(Query::CreatePerson {
                    last_name: String::from(last_name),
                    first_name: first_name,
                });
            }
            Some("show_all_persons") => return Ok(Query::ShowAllPersons),
            Some("person_info") => {
                let id = match command.next() {
                    Some(id) => String::from(id),
                    None => {
                        let message =
                            Query::err_message("person_id is missing", "person_info <person_id>");
                        return Err(message);
                    }
                };
                return Ok(Query::ShowPersonInfo { person_id: id });
            }
            Some("delete_person") => {
                let id: String = match command.next() {
                    Some(id) => String::from(id),
                    None => {
                        let message =
                            Query::err_message("person_id is missing", "delete_person <person_id>");
                        return Err(message);
                    }
                };
                return Ok(Query::DeletePerson { person_id: id });
            }
            Some("add_person") => {
                let person_id = match command.next() {
                    Some(id) => String::from(id),
                    None => {
                        let message = Query::err_message(
                            "person_id is missing",
                            "add_person <person_id> <department_id>",
                        );
                        return Err(message);
                    }
                };
                let department_id = match command.next() {
                    Some(id) => String::from(id),
                    None => {
                        let message = Query::err_message(
                            "department_id is missing",
                            "add_person <person_id> <department_id>",
                        );
                        return Err(message);
                    }
                };
                return Ok(Query::AddPersonToDepartment {
                    person_id: person_id,
                    department_id: department_id,
                });
            }
            Some("remove_person") => {
                let person_id = match command.next() {
                    Some(id) => String::from(id),
                    None => {
                        let message = Query::err_message(
                            "person_id is missing",
                            "remove_person <person_id> <department_id>",
                        );
                        return Err(message);
                    }
                };
                let department_id = match command.next() {
                    Some(id) => String::from(id),
                    None => {
                        let message = Query::err_message(
                            "department_id is missing",
                            "remove_person <person_id> <department_id>",
                        );
                        return Err(message);
                    }
                };
                return Ok(Query::RemovePersonFromDepartment {
                    person_id: person_id,
                    department_id: department_id,
                });
            }
            // TODO
            Some("all_info") => return Ok(Query::ShowAllPersonsByDepartment),
            Some("help") => return Ok(Query::Help),
            Some("quit") => return Ok(Query::Quit),
            _ => return Err(String::from("unrecognized command!")),
        }
    }

    pub fn print_help() {
        let command_list: Vec<&str> = vec![
            "create_department <department_name>              : create a new department and add it to the department list",
            "show_all_departments                             : show all departments in the list",
            "department_info <department_id>                  : show an information of the department",
            "delete_department <department_id>                : delete a department from the list",
            "create_person <last_name> <first_name>(optional) : create a new person and add it to the person list",
            "show_all_persons                                 : show all persons in the list",
            "person_info <person_id>                          : show an information of the person",
            "delete_person <person_id>                        : delete a person from the list",
            "add_person <person_id> <department_id>           : add a person to the department",
            "remove_person <person_id> <department_id>        : remove a person from the department",
            "all_info                                         : show all persons by department",
            "help                                             : show help ",
            "quit                                             : finish this application",
        ];
        println!("~~HELP~~");
        for text in command_list {
            println!("{}", text);
        }
    }

    fn err_message(message: &str, how_to_use: &str) -> String {
        let mut err_message = String::new();
        err_message.push_str(message);
        err_message.push_str("\nhow to use:\n ");
        err_message.push_str(how_to_use);
        return err_message;
    }
}
