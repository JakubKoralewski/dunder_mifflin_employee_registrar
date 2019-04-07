use std::{
    io, 
    collections::HashMap, 
    collections::hash_map::Entry
};
/* From this assignment: https://doc.rust-lang.org/book/ch08-03-hash-maps.html#summary */

pub fn start() {
    println!("Hello there! This is the Dunder Mifflin employee registrar. Feel free to start typing commands :)");
    println!("Example commands: `Add Jim to Sales`, `Add Dwight to Assistant to the Regional Manager`");
    let mut registrar: HashMap<String, Vec<String>> = HashMap::new();
    
    loop {
        let mut commands_raw = String::new();
        
        io::stdin().read_line(&mut commands_raw).expect("Failed to read line");
        let commands: Vec<&str> = commands_raw.split_whitespace().collect();
        if commands.len() == 0 {
            println!("Seems like you provided no input! Try again.");
            continue;
        }

        // Example commands: ["Add", "Jim", "to", "Sales"]

        match &commands[0].to_lowercase()[..] {
            "add" => {
                // e.g.: `add Jim to Sales`
                let employees = registrar.entry(commands[3].to_string()).or_insert(Vec::new());
                employees.push(commands[1].to_string());
            },
            "get" => {
                // e.g.: `get all in Sales`
                let employees_entry = registrar.entry(commands[3].to_string());
                match employees_entry {
                    Entry::Occupied(o) => {
                        let mut employees = o.into_mut().clone();
                        employees.sort();
                        print!("All employees in {}: {:?}", &commands[3], &employees);
                        print!("\n");
                    },
                    Entry::Vacant(_) => {
                        println!("Department does not exist! Try again.");
                        continue;
                    },
                }
            },
            "remove" => {
                // e.g.: Remove Pamela from Sales
                let employees_entry = registrar.entry(commands[3].to_string());
                match employees_entry {
                    Entry::Occupied(o) => {
                        let employees = o.into_mut();
                        let index = match employees.iter().position(|employee| *employee == commands[1]) {
                            Some(num) => num,
                            None => {
                                println!("{} does not exist in {}", commands[1], commands[3]);
                                continue;
                            }
                        };
                        employees.remove(index);
                        print!("All employees in {}: {:?}", &commands[3], &employees);
                        print!("\n");
                    },
                    Entry::Vacant(_) => {
                        println!("Department `{}`does not exist! Try again.", &commands[3]);
                        continue;
                    },
                }
            }
            _ => {
                println!("Seems like you didn't specify a valid operator, e.g.: `Add`, `Get`, `Remove`.");
            }
        }
    }
}