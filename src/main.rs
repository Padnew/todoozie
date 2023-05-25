#![allow(unused_variables, dead_code)]
use todoozie::Todo;
use todoozie::{self, timeout1s};

fn main() {
    let mut incomplete_todos: Vec<Todo> = Vec::<Todo>::new();
    let menu_choice: i32 = 0;
    while menu_choice != 4 {
        todoozie::clear_terminal();
        println!("Todoozie v1.0");
        println!("1. Add new Todo");
        println!("2. View remaining Todos");
        println!("3. Clear all Todos");
        println!("4. Exit Todoozie");
        println!("____________________________________________");
        println!("Please enter your choice below: (1, 2, 3, 4)");
        let menu_choice: i32 = todoozie::get_int_input();

        match menu_choice {
            1 => todoozie::create_new_todo(&mut incomplete_todos),
            2 => todoozie::view_remaining_todos(&mut incomplete_todos),
            3 => todoozie::clear_todos(&mut incomplete_todos),
            4 => std::process::exit(1),
            _ => {
                println!("Invalid input");
                timeout1s();
                continue;
            }
        };
    }
}
