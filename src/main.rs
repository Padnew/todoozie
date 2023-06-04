#![allow(unused_variables, dead_code)]
use colored::Colorize;
use todoozie::Todo;
use todoozie::{self, timeout1s};
fn main() {
    let mut incomplete_todos: Vec<Todo> = Vec::<Todo>::new();
    let menu_choice: i32 = 0;
    while menu_choice != 4 {
        todoozie::clear_terminal();
        println!("{}", "Todoozie v1.0".bright_magenta());
        println!(
            "{} {}",
            "1. ".bright_magenta(),
            "Add new Todo".bright_green()
        );
        println!(
            "{} {}",
            "2. ".bright_magenta(),
            "View remaining Todos".bright_green()
        );
        println!(
            "{} {}",
            "3. ".bright_magenta(),
            "Clear all Todos".bright_green()
        );
        println!(
            "{} {}",
            "4. ".bright_magenta(),
            "Exit Todoozie".bright_green()
        );
        println!(
            "{}",
            "____________________________________________".bright_magenta()
        );
        println!(
            "{}",
            "Please enter your choice below: (1, 2, 3, 4)".bright_magenta()
        );
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
