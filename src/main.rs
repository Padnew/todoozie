#![allow(unused_variables, dead_code)]
use std::io;

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
    let incomplete_todos: Vec<Todo> = Vec::<Todo>::new();
    display_menu(incomplete_todos);
}

fn display_menu(incomplete_todos: Vec<Todo>) {
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
        _ => panic!("Unexpected menu input"),
    };
}

fn create_new_todo(mut incomplete_todos: Vec<Todo>) {
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
}

fn display_todos(incomplete_todos: Vec<Todo>) {
    println!("Current Todos:");
    // loop {
    //     println!("TODO: {} | Status: {}", incomplete_todos.todo_task_name, todo_status)
    // }
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
    return user_input.to_string();
}

fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}
