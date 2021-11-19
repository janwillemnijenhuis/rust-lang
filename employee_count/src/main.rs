use std::io::stdin;
use std::collections::HashMap;
use std::cmp::Ordering;


pub fn main() {
    let mut exit: bool = false;
    let mut employee_dict = HashMap::new();
    while(!exit) {
        write_menu();
        let mut choice = String::new();
        stdin.read_line(&choice).expect("Please provide a valid input argument");
        if choice == "NEW"{
            print!("Add: ");
            let mut name = String::new();
            let mut department = String::new();
            stdin.read_line(&mut name).expect("Please provide a valid first name");
            print!(" to ");
            stdin.read_line(&mut department).expect("Please provide a valid department");
            employee_dict.insert(name, department);
        }
        else if choice == "LIST" {
            println!("From which department would you like to retrieve the list of employees?");
            println!("Please type one of the departments listed below:");
            let mut departments_list: Vec<String> = Vec::new();
            departments_list = give_departments(employee_dict);
            println!("{:?}", departments_list);
            let mut dep_choice: String = String::new();
            stdin.read_line(&mut dep_choice).expect("Please provide a valid department");
            retrieve_sorted_list(dep_choice, employee_dict);
        }
        else if choice == "EXIT" {
            exit = true;
        }
        else {
            println!("Error: not a valid input. Please try again following the instructions below.");
        }
    }
}

pub fn write_menu() {
    println!("Syntax:");
    println!("1. To add a new employee, type:                   NEW");
    println!("2. To retrieve the current employee list, type:   LIST");
    println!("3. To exit, type:                                 EXIT");
}

pub fn give_departments(h: &mut HashMap <&str, &str>) -> Vec<String> {
    let mut departments_list: Vec<String> = Vec::new();
    for (key, value) in h {
        if !departments_list.contains(value) {
            departments_list.push(value);
        }
    }
    return departments_list;
}

pub fn retrieve_sorted_list(department: &str, h: &mut HashMap <&str, &str>) -> Vec<String> {
    let mut employee_list: Vec<String> = Vec::new();
    for (key, value) in h {
        if value == department {
            employee_list.push(key);
        }
    }
    sort_list(employee_list);
}

pub fn sort_list(l: &mut Vec<&str>) -> Vec<String> {
    let mut sorted_list: Vec<String> = Vec::new();
    let mut list_len: usize = l.len();
    for i in 0..(list_len - 2) {
        let mut p1: String = l[i];
        let mut p2: String = l[i+1];
        if p1.cmp(p2) == Ordering::Greater {
            let mut temp: String = p1;
            p1 = p2;
            p2 = temp;
        }
    }
}

