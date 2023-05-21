#![allow(unused_variables, dead_code)]
use std::io;
use std::{thread, time};

#[derive(Debug)]
enum TodoStatus {
    Incomplete,
    Complete,
}

#[derive(Debug)]
struct Todo {
    todo_status: TodoStatus,
    todo_task_name: String,
}

fn main() {
    let mut incomplete_todos: Vec<Todo> = Vec::<Todo>::new();
    menu(&mut incomplete_todos);
}

fn menu(incomplete_todos: &mut Vec<Todo>) {
    let menu_choice: i32 = 0;
    while menu_choice != 4 {
        clear_terminal();
        println!("Todoozie v1.0");
        println!("1. Add new Todo");
        println!("2. View remaining Todos");
        println!("3. Clear all Todos");
        println!("4. Exit Todoozie");
        println!("____________________________________________");
        println!("Please enter your choice below: (1, 2, 3, 4)");
        let menu_choice: i32 = get_int_input();

        match menu_choice {
            1 => create_new_todo(incomplete_todos),
            2 => view_remaining_todos(incomplete_todos),
            3 => clear_todos(incomplete_todos),
            4 => std::process::exit(1),
            _ => continue,
        };
    }
}

fn create_new_todo(incomplete_todos: &mut Vec<Todo>) {
    clear_terminal();
    println!("Add new Todo");
    println!("_____________________");
    println!("Task name: ");
    let task_name = get_string_input();

    let new_todo = Todo {
        todo_status: TodoStatus::Incomplete,
        todo_task_name: task_name,
    };
    incomplete_todos.push(new_todo);
    println!("Todo created, returning to menu...");
    thread::sleep(time::Duration::from_millis(1000));
}

fn view_remaining_todos(incomplete_todos: &mut Vec<Todo>) {
    clear_terminal();
    println!("Current Todos:");
    for todo in incomplete_todos {
        println!(
            "TODO: {} | Status: {:?}",
            todo.todo_task_name, todo.todo_status
        )
    }
    println!("Press enter to return to the menu");
    get_string_input();
}

fn clear_todos(incomplete_todos: &mut Vec<Todo>) {
    clear_terminal();
    println!("Are you sure you wish to clear all current todos? (Type 'yes' to confirm)");
    let double_check = &get_string_input() as &str;
    match double_check.trim() {
        "yes" => {
            incomplete_todos.clear();
            println!("Todos cleared, returning to menu...");
            thread::sleep(time::Duration::from_millis(1000));
        }
        _ => {
            println!("Todos NOT cleared, returning to menu...");
            thread::sleep(time::Duration::from_millis(1000));
        }
    }
}

// ____________________________Input and terminal functions_____________________________

fn get_int_input() -> i32 {
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Unable to read input");
    let user_input: i32 = match user_input.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Error: User exited program or invalid input"),
    };
    return user_input;
}

fn get_string_input() -> String {
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Unable to read input");
    return user_input;
}

fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}
