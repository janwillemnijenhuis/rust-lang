use integer_list::return_mean;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::stdin;
const EXIT: &str = "EXIT";
const NEW: &str = "NEW";
const LIST: &str = "LIST";

pub fn main() {
    let mut exit: bool = false;
    let mut employee_dict: HashMap<String, String> = HashMap::new();
    while !exit {
        write_menu();
        let mut choice: String = String::new();
        stdin()
            .read_line(&mut choice)
            .expect("Please provide a valid input argument");
        let choice = choice.trim();
        if choice == NEW {
            println!("Add: ");
            let mut name = String::new();
            let mut department = String::new();
            stdin()
                .read_line(&mut name)
                .expect("Please provide a valid first name");
            println!("To: ");
            stdin()
                .read_line(&mut department)
                .expect("Please provide a valid department");
            employee_dict.insert(name, department);
        } else if choice == LIST {
            println!("From which department would you like to retrieve the list of employees?");
            println!("Please type one of the departments listed below:");
            let mut departments_list: Vec<String> = Vec::new();
            departments_list = give_departments(&mut employee_dict);
            println!("{:#?}", departments_list);
            let mut dep_choice: String = String::new();
            stdin()
                .read_line(&mut dep_choice)
                .expect("Please provide a valid department");
            retrieve_sorted_list(dep_choice, &mut employee_dict);
        } else if choice == EXIT {
            exit = true;
        } else {
            println!(
                "Error: not a valid input. Please try again following the instructions below."
            );
        }
    }
}

pub fn write_menu() {
    println!("Syntax:");
    println!("1. To add a new employee, type:                   NEW");
    println!("2. To retrieve the current employee list, type:   LIST");
    println!("3. To exit, type:                                 EXIT");
}

pub fn give_departments(h: &mut HashMap<String, String>) -> Vec<String> {
    let mut departments_list: Vec<String> = Vec::new();
    for (key, value) in h {
        if !departments_list.contains(&value.to_string()) {
            departments_list.push(value.to_string());
        }
    }
    return departments_list;
}

pub fn retrieve_sorted_list(department: String, h: &mut HashMap<String, String>) {
    let mut employee_list: Vec<String> = Vec::new();
    for (key, value) in h {
        if value.to_string() == department {
            employee_list.push(key.to_string());
        }
    }
    sort_list(&mut employee_list);
    println!("{:#?}", employee_list);
}

pub fn sort_list(l: &mut Vec<String>) {
    let list_len: usize = l.len();
    for i in 0..(list_len - 1) {
        for j in 0..(list_len - 1) {
            if l[j].cmp(&l[j + 1]) == Ordering::Greater {
                let mut temp: String = l[j].clone();
                l[j] = l[j + 1].clone();
                l[j + 1] = temp;
            }
        }
    }
}
