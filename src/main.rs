#![allow(unused_variables, dead_code)]

use colored::Colorize;
use todoozie::Todo;
use todoozie::{self, timeout1s};
fn main() {
    let mut todo_list: Vec<Todo> = Vec::<Todo>::new();
    let menu_choice: i32;
    todoozie::read_todos_from_file(&mut todo_list);
    loop {
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
            "View all Todos".bright_green()
        );
        println!(
            "{} {}",
            "3. ".bright_magenta(),
            "Complete a Todo".bright_green()
        );
        println!(
            "{} {}",
            "4. ".bright_magenta(),
            "Clear all Todos".bright_green()
        );
        println!(
            "{} {}",
            "5. ".bright_magenta(),
            "Exit and save Todoozie".bright_green()
        );
        println!(
            "{}",
            "____________________________________________".bright_magenta()
        );
        println!(
            "{}",
            "Please enter your choice below (1, 2, 3, 4, 5):".bright_magenta()
        );

        let menu_choice = todoozie::get_int_input();

        match menu_choice {
            1 => todoozie::create_new_todo(&mut todo_list),
            2 => todoozie::view_all_todos(&todo_list),
            3 => todoozie::complete_remaining_todos(&mut todo_list),
            4 => todoozie::clear_todos(&mut todo_list),
            5 => todoozie::write_todos_to_file(&mut todo_list),

            _ => {
                println!("Invalid input");
                timeout1s();
                continue;
            }
        };
    }
}
