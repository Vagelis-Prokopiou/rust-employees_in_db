#[macro_use]
extern crate mysql;

use mysql::QueryResult;

mod helpers;

fn main() {
    // Database info.
    let db_user = "root";
    let db_password = "root";
    let db_name = "rust_employees";
    let db_ip = "localhost";
    let db_port = "3306";

    let db_connection = mysql::Pool::new(format!("mysql://{}:{}@{}:{}/{}", db_user, db_password, db_ip, db_port, db_name)).unwrap();

    // Create a table.
    db_connection.prep_exec(r"CREATE TABLE IF NOT EXISTS employees (
                         employee_id INT NOT NULL AUTO_INCREMENT KEY,
                         name VARCHAR(255) NOT NULL,
                         department VARCHAR(255) NOT NULL
                         )", ()).unwrap();

    loop {
        println!();
        println!("What would you like to do?");
        println!("Press 1 to add a new employee.");
        println!("Press 2 to retrieve all existing employees per department.");
        println!("Press 3 to retrieve all existing employees of a specific department.");
        println!("Press q to quit.");
        println!();
        println!("Please provide your selection:");

        let user_action: String = helpers::get_user_input();

        if user_action == "1" {
            println!("Please provide the new employee. E.g.: \"Add George to Sales\"");

            let new_employee: String = helpers::get_user_input();
            let tokens: Vec<&str> = new_employee.trim().split(' ').collect();

            if !helpers::is_valid_employee_provided(&tokens) {
                println!("Wrong format. Try \"Add George to Sales\"");
                continue;
            }

            db_connection.prep_exec(r"INSERT INTO employees (name, department)
                                   VALUES (:name, :department)", params! {
                "name" => tokens[1],
                "department" => tokens[3],
            }).unwrap();
        } else if user_action == "2" {
            let mut result: QueryResult = db_connection.prep_exec(r"SELECT name, department FROM employees ORDER BY DEPARTMENT, NAME", ()).unwrap();
            while result.more_results_exists() {
                for x in result.by_ref() {
                    match x {
                        Ok(row) => println!("Row: {:?}", row), /* Todo: Get fields from row. */
                        Err(e) => println!("Error: {:?}", e),
                    }
                }
            }
        } else if user_action == "3" {
            println!("Please provide the desired department. The available departments are:");
            let department: String = helpers::get_user_input();
            println!("Department: {}", department);
        } else if user_action == "q" {
            break;
        } else {
            println!("Wrong selection. Please, try again.");
        }
    }
}