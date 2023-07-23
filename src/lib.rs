#![allow(unused_must_use)]
use colored::Colorize;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
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
    pub fn build(task_details: String, todo_list: &mut Vec<Todo>) -> Result<(), ()> {
        let split: std::str::Split<&str> = task_details.split(" ");
        let collection: Vec<&str> = split.collect();
        if collection.len() < 2 {
            println!("{}", "Invalid number of arguments, try again...".red());
            timeout1s();
            create_new_todo(todo_list);
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
        match task_urgency.trim() {
            "1" | "Urgent" => Ok(todo_list.push(Todo {
                todo_status: (TodoStatus::Incomplete),
                todo_task_name: (task_name),
                todo_urgency: (TodoUrgency::Urgent),
            })),
            "2" | "Passive" => Ok(todo_list.push(Todo {
                todo_status: (TodoStatus::Incomplete),
                todo_task_name: (task_name),
                todo_urgency: (TodoUrgency::Passive),
            })),
            "3" | "Reminder" => Ok(todo_list.push(Todo {
                todo_status: (TodoStatus::Incomplete),
                todo_task_name: (task_name),
                todo_urgency: (TodoUrgency::Reminder),
            })),
            _ => Err({
                println!("{}", "Invalid task severity, try again...".red());
                timeout1s();
                create_new_todo(todo_list);
            }),
        }
    }
}

pub fn create_new_todo(todo_list: &mut Vec<Todo>) {
    clear_terminal();
    println!("{}", "Add new Todo".bright_magenta());
    println!(
        "{}",
        "_____________________________________________________________".bright_magenta()
    );
    println!(
        "Task and severity (1. {}, 2. {}, 3. {}): ",
        "Urgent".red(),
        "Passive".yellow(),
        "Reminder".green()
    );
    Todo::build(get_string_input(), todo_list);
    println!("Todo created, returning to menu...");
    timeout1s()
}

pub fn view_all_todos(todo_list: &Vec<Todo>) {
    clear_terminal();
    if todo_list.len() == 0 {
        println!(
            "{}",
            "Congratz you finished all your todos\nPress enter to return to the menu"
                .bright_green()
        );
        get_string_input();
    } else {
        println!("{}", "Current Todos:".bright_magenta());
        println!(
            "{}",
            "_____________________________________________________________".bright_magenta()
        );
        for todo in todo_list {
            println!(
                "TODO: {} | Status: {} | Urgency: {}",
                todo.todo_task_name.blue(),
                match todo.todo_status {
                    TodoStatus::Incomplete => todo.todo_status.to_string().red(),
                    TodoStatus::Complete => todo.todo_status.to_string().green(),
                },
                match todo.todo_urgency {
                    TodoUrgency::Urgent => todo.todo_urgency.to_string().red(),
                    TodoUrgency::Passive => todo.todo_urgency.to_string().yellow(),
                    TodoUrgency::Reminder => todo.todo_urgency.to_string().green(),
                }
            )
        }
        println!("{}", "Press enter to return to the menu".red());
        get_string_input();
    }
}

pub fn clear_todos(todo_list: &mut Vec<Todo>) {
    clear_terminal();
    if todo_list.len() == 0 {
        println!(
            "{}",
            "No todos to clear\nPress enter to return to the menu".bright_green()
        );
        get_string_input();
    } else {
        println!(
            "{}",
            "Are you sure you wish to clear all current todos? (Type 'yes' to confirm)".red()
        );
        let double_check = get_string_input();
        match double_check.trim() {
            "yes" => {
                todo_list.clear();
                println!("Todos cleared, returning to menu...");
                timeout1s()
            }
            _ => {
                println!("{}", "Todos NOT cleared, returning to menu...".red());
                timeout1s()
            }
        }
    }
}

pub fn get_int_input() -> i32 {
    loop {
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input);
        let _user_input: i32 = match user_input.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("{}", "Invalid input, please try again".red());
                continue;
            }
        };
    }
}

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

pub fn read_todos_from_file(mut todo_list: &mut Vec<Todo>) {
    if let Ok(file) = File::open("data/todos.txt") {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                Todo::build(line, &mut todo_list);
            } else {
                println!("Failed to read line");
            }
        }
    }
}

fn get_remaining_todos(todo_list: &mut Vec<Todo>) -> Vec<&mut Todo> {
    return todo_list
        .into_iter()
        .filter(|x| x.todo_status == TodoStatus::Incomplete)
        .collect();
}

pub fn write_todos_to_file(todo_list: &mut Vec<Todo>) {
    clear_terminal();
    println!(
        "{}",
        "Exit and save remaining todos? ('y' for yes, 'n' to exit without saving, anything else to return to menu)\n(Any completed todos wont be saved to file)"
    );
    let exit_bool = get_string_input();
    match exit_bool.trim() {
        "y" => {
            let rem_todos = get_remaining_todos(todo_list);
            let mut f = File::create("data/todos.txt").expect("Unable to find/create file");
            for todo in &rem_todos {
                write!(f, "{} {}\n", todo.todo_task_name, todo.todo_urgency);
            }
            std::process::exit(1);
        }
        "n" => std::process::exit(1),
        _ => {
            println!("{}", "Returning to menu...".red());
            timeout1s();
        }
    }
}

pub fn complete_remaining_todos(todo_list: &mut Vec<Todo>) {
    clear_terminal();
    let mut index: i32 = 0;
    let mut rem_todos = get_remaining_todos(todo_list);

    if rem_todos.len() == 0 {
        println!(
            "{}",
            "No remaining todos\nPress enter to return to the menu".red()
        );
        get_string_input();
    } else {
        println!("{}", "Which todo have you completed?".bright_magenta());
        for todo in &rem_todos {
            index += 1;
            println!(
                "{} {}",
                { index.to_string() + "." }.bright_magenta(),
                todo.todo_task_name.green()
            );
        }
        println!(
            "{}",
            "____________________________________________".bright_magenta()
        );
        println!(
            "{}",
            "Please enter the number of the Todo you have completed:".red()
        );
        index = 0;
        let user_input = get_int_input();
        for mut todo in &mut rem_todos {
            index += 1;
            if index == user_input {
                todo.todo_status = TodoStatus::Complete;
                println!("Todo status updated, returning to menu...");
                timeout1s();
            }
        }
    }
}
