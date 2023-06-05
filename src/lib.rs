#![allow(unused_must_use)]
use colored::Colorize;
use std::{fmt, io};
use std::{thread, time};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TodoStatus {
    Incomplete,
    Complete,
}
impl fmt::Display for TodoStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TodoStatus::Incomplete => write!(f, "Incomplete"),
            TodoStatus::Complete => write!(f, "Complete"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TodoUrgency {
    Urgent,
    Passive,
    Reminder,
}
impl fmt::Display for TodoUrgency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TodoUrgency::Passive => write!(f, "Passive"),
            TodoUrgency::Reminder => write!(f, "Reminder"),
            TodoUrgency::Urgent => write!(f, "Urgent"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Todo {
    pub todo_status: TodoStatus,
    pub todo_task_name: String,
    pub todo_urgency: TodoUrgency,
}

impl Todo {
    pub fn build(task_details: String, incomplete_todos: &mut Vec<Todo>) -> Result<(), ()> {
        let split: std::str::Split<&str> = task_details.split(" ");
        let collection: Vec<&str> = split.collect();
        if collection.len() < 2 {
            println!("{}", "Invalid number of arguments, try again...".red());
            timeout1s();
            create_new_todo(incomplete_todos);
        } else {
        }
        // 'tn is the name of the block expression so it can break out easily
        let task_name = 'tn: {
            let mut name = "".to_owned();
            let mut count = 0;
            loop {
                if count == collection.len() - 1 {
                    break 'tn name;
                } else if count < collection.len() - 2 {
                    name.push_str(collection[count]);
                    name.push_str(" ");
                } else {
                    name.push_str(collection[count]);
                }
                count += 1;
            }
        };
        let task_urgency = collection[collection.len() - 1];
        // println!("Task name: {}, Task severity: {}", task_name, task_urgency);
        match task_urgency.trim() {
            "1" => Ok(incomplete_todos.push(Todo {
                todo_status: (TodoStatus::Incomplete),
                todo_task_name: (task_name),
                todo_urgency: (TodoUrgency::Urgent),
            })),
            "2" => Ok(incomplete_todos.push(Todo {
                todo_status: (TodoStatus::Incomplete),
                todo_task_name: (task_name),
                todo_urgency: (TodoUrgency::Passive),
            })),
            "3" => Ok(incomplete_todos.push(Todo {
                todo_status: (TodoStatus::Incomplete),
                todo_task_name: (task_name),
                todo_urgency: (TodoUrgency::Reminder),
            })),
            _ => Err({
                println!("{}", "Invalid task severity, try again...".red());
                timeout1s();
                create_new_todo(incomplete_todos);
            }),
        }
    }
}

pub fn create_new_todo(incomplete_todos: &mut Vec<Todo>) {
    clear_terminal();
    println!("{}", "Add new Todo".bright_magenta());
    println!("_________________________________________________________");
    println!(
        "Task and severity (1. {}, 2. {}, 3. {}): ",
        "Urgent".red(),
        "Passive".green(),
        "Reminder".yellow()
    );
    Todo::build(get_string_input(), incomplete_todos);
    println!("Todo created, returning to menu...");
    timeout1s()
}

pub fn view_all_todos(incomplete_todos: &mut Vec<Todo>) {
    clear_terminal();
    println!("{}", "Current Todos:".bright_magenta());
    println!("_________________________________________________________");
    for todo in incomplete_todos {
        println!(
            "TODO: {} | Status: {} | Urgency: {}",
            todo.todo_task_name.blue(),
            todo.todo_status.to_string().red(),
            match todo.todo_urgency {
                TodoUrgency::Passive => todo.todo_urgency.to_string().green(),
                TodoUrgency::Reminder => todo.todo_urgency.to_string().yellow(),
                TodoUrgency::Urgent => todo.todo_urgency.to_string().red(),
            }
        )
    }
    println!("{}", "Press enter to return to the menu".red());
    get_string_input();
}

pub fn clear_todos(incomplete_todos: &mut Vec<Todo>) {
    clear_terminal();
    println!(
        "{}",
        "Are you sure you wish to clear all current todos? (Type 'yes' to confirm)".red()
    );
    let double_check = get_string_input();
    match double_check.trim() {
        "yes" => {
            incomplete_todos.clear();
            println!("Todos cleared, returning to menu...");
            timeout1s()
        }
        _ => {
            println!("{}", "Todos NOT cleared, returning to menu...".red());
            timeout1s()
        }
    }
}
// TODO: Try to replace this with text_IO crate
// pub fn get_int_input() -> i32 {
//     let mut user_input = String::new();
//     io::stdin().read_line(&mut user_input);
//     let user_input: i32 = match user_input.trim().parse() {
//         Ok(num) => num,
//         Err(_) => panic!("Invalid input"),
//     };
//     return user_input;
// }

pub fn get_string_input() -> String {
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Unable to read input somehow even though String input is really robust");
    return user_input;
}

pub fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn timeout1s() {
    thread::sleep(time::Duration::from_millis(1000));
}
