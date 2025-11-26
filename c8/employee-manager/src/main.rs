/*
 * AUTHOR: toydotgame
 * CREATED: 2025-11-26
 * Using a hash map and vectors, create a text interface to allow a user to add
 * employee names to a department in a company; for example, “Add Sally to
 * Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of
 * all people in a department or all people in the company by department,
 * sorted alphabetically.
 *
 * My implementation has a few flaws/potential improvements/lack of
 * idiomaticness:
 * - Department names, namespaced IDs, and employee name list vectors are all
 *   stored using literals and case-by-case `match` statements that just…suck.
 *   They should ideally be managed in a much, much nicer way that allows for
 *   better and dynamic access
 * - Said employee list vectors are mutable and sorted in place EVERY TIME a
 *   user requests a listing of it. Ideally the names should be sorted on
 *   insert (if we can guarantee ALL inserts are through this program) or just…
 *   with a little more thought and care next time, at least
 * - Help printing and managing of generic errors and command parsing (etc)
 *   SUCKS SO BAD!!!!!!!!!! Oh well
 * - The use of enums and String vs &str and also my format string usage is
 *   ABSOLUTELY careless and without thought…it should be actually thought
 *   about if/when refactored
 *
 * This is all Rust and therefore, despite SUCKING, is the fastest code I've
 * ever written for a program of this (admittedly modest) functionality level.
 * So lol.
 */

use std::io::{self, Write};

enum Command {
    Add { name: String, department: String },
    ListAll,
    ListDepartment { department: String },
    Exit,
    Invalid { reason: String },
}

const DEPARTMENTS: [&str; 4] = ["accounting", "engineering", "hr", "sales"];

fn main() {
    println!("==== EMPLOYEE MANAGER ====");
    println!(
        "Commands:\n\
        \tadd <name> to <department_id>\n\
        \t\tAdds the name <name> to the department with ID <department_id>\n\
        \tlist <all|department <department_id>>\n\
        \t\t`list all` lists all people in the company by department, then \
        alphabetically\n\
        \t\t`list department <department_id>` will list all people in the \
        department with ID <department_id> alphabetically\n\
        \texit\n\
        \t\tQuits\n"
    );
    println!(
        "Available departments:\n\
        \taccounting\n\
        \tengineering\n\
        \thr\n\
        \tsales\n"
    );
    // This sucks:
    let mut accounting_department: Vec<String> = Vec::new();
    let mut engineering_department: Vec<String> = Vec::new();
    let mut hr_department: Vec<String> = Vec::new();
    let mut sales_department: Vec<String> = Vec::new();

    loop {
        // Command loop
        let command: String = match prompt() {
            Ok(input) => input,
            Err(_) => {
                eprintln!("Error reading input!");
                continue;
            }
        };
        match parse_command(command) {
            // I _believe_(?) the below "consumes ..." comments are true. Not actually sure how
            // enums are handled re. ownership
            Command::Add { name, department } => match department.as_str() {
                // Consumes `department`
                "accounting" => accounting_department.push(name), // Consumes `name`
                "engineering" => engineering_department.push(name),
                "hr" => hr_department.push(name),
                "sales" => sales_department.push(name),
                invalid => panic!("Tried to add employee to department that doesn't exist! (Department was \"{invalid}\")"),
            },
            Command::Exit => return, // Exit 0
            Command::Invalid { reason } => println!("Invalid: {reason}!"),
            Command::ListAll => {
                println!("Accounting:");
                accounting_department.sort_unstable();
                for employee in &accounting_department {
                    println!("\t{employee}");
                }

                println!("\nEngineering:");
                engineering_department.sort_unstable();
                for employee in &engineering_department {
                    println!("\t{employee}");
                }
                println!("\nHR:");
                hr_department.sort_unstable();
                for employee in &hr_department {
                    println!("\t{employee}");
                }
                println!("\nSales:");
                sales_department.sort_unstable();
                for employee in &sales_department {
                    println!("\t{employee}");
                }
            },
            Command::ListDepartment { department } => match department.as_str() {
                "accounting" => {
                    accounting_department.sort_unstable();
                    for employee in &accounting_department {
                        println!("\t{employee}");
                    }
                },
                "engineering" => {
                    engineering_department.sort_unstable();
                    for employee in &engineering_department {
                        println!("\t{employee}");
                    }
                },
                "hr" => {
                    hr_department.sort_unstable();
                    for employee in &hr_department {
                        println!("\t{employee}");
                    }
                },
                "sales" => {
                    sales_department.sort_unstable();
                    for employee in &sales_department {
                        println!("\t{employee}");
                    }
                },
                invalid => panic!("Tried to list department that doesn't exist! (Department was \"{invalid}\")"),
            }
        };
    }
}

fn prompt() -> Result<String, io::Error> {
    print!("> ");
    io::stdout()
        .flush()
        .expect("Failed to flush stdout when printing command prompt!");

    let mut input: String = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => Ok(input.trim().to_ascii_lowercase()),
        Err(e) => Err(e),
    }
}

fn parse_command(command: String) -> Command {
    let mut command = command.split(' ');
    match command.next() {
        Some("add") => {
            let name: String = match command.next() {
                Some(name) => String::from(name),
                None => {
                    return Command::Invalid {
                        reason: String::from("No name specified"),
                    }
                }
            };
            if command.next() != Some("to") {
                return Command::Invalid {
                    reason: String::from("Missing \"to\" after employee name"),
                };
            }
            let department: String = match command.next() {
                Some(department) => {
                    if !DEPARTMENTS.contains(&department) {
                        return Command::Invalid {
                            reason: format!("The department \"{department}\" does not exist"),
                        };
                    }
                    String::from(department)
                }
                None => {
                    return Command::Invalid {
                        reason: String::from("No department specified"),
                    }
                }
            };
            if command.next() != None {
                return Command::Invalid {
                    reason: String::from("Too many arguments for `add`"),
                };
            }

            Command::Add {
                name: name,
                department: department,
            }
        }
        Some("list") => match command.next() {
            Some("all") => Command::ListAll,
            Some("department") => {
                let department: String = match command.next() {
                    Some(department) => {
                        if !DEPARTMENTS.contains(&department) {
                            return Command::Invalid {
                                reason: format!("The department \"{department}\" does not exist"),
                            };
                        }
                        String::from(department)
                    }
                    None => {
                        return Command::Invalid {
                            reason: String::from("No department specified"),
                        }
                    }
                };
                if command.next() != None {
                    return Command::Invalid {
                        reason: String::from("Too many arguments for `department`"),
                    };
                }

                Command::ListDepartment {
                    department: department,
                }
            }
            Some(_) | None => Command::Invalid {
                reason: String::from("Invalid or no operation for `list` command"),
            },
        },
        Some("exit") => {
            if command.next() == None {
                return Command::Exit;
            }
            Command::Invalid {
                reason: String::from("Too many arguments for `exit` command"),
            }
        }
        Some(_) | None => Command::Invalid {
            reason: String::from("Unknown/no command! Use `exit` to exit"),
        },
    }
}
