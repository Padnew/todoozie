use std::fs;
use std::io;
use std::{thread, time};

#[derive(Debug)]
pub enum TodoStatus {
    Incomplete,
    Complete,
}

#[derive(Debug)]
pub struct Todo {
    pub todo_status: TodoStatus,
    pub todo_task_name: String,
}

pub fn create_new_todo(incomplete_todos: &mut Vec<Todo>) {
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

pub fn view_remaining_todos(incomplete_todos: &mut Vec<Todo>) {
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

pub fn clear_todos(incomplete_todos: &mut Vec<Todo>) {
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
pub fn get_int_input() -> i32 {
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

pub fn get_string_input() -> String {
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Unable to read input");
    return user_input;
}

pub fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}

// pub fn read_data() {
//     let file = "src/file.txt";
//     let file_text = fs::read_to_string(file).expect("Error in reading file contents");
// }
