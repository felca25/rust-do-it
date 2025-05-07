pub mod db;
pub mod print_tools;
pub mod task;
pub mod task_list;

use db::TodoList;
use print_tools::{clear, line};
use rusqlite::Error;

fn main() -> Result<(), Error> {
    let result = TodoList::new();
    let mut todo_list;
    match result {
        Ok(value) => {
            todo_list = value;
        }
        Err(e) => {
            panic!("Error: {}", e);
        }
    }

    clear();
    todo_list.print()?;
    line();
    let mut stop = false;
    // starting main loop
    while !stop {
        todo_list.print_home(&mut stop)?;
    }
    std::process::Command::new("clear").status().unwrap();
    Ok(())
}
