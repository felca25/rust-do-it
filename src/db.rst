use crate::print_tools::{clear, line};
use crate::task::Task;
use rusqlite::{Connection, Error, OptionalExtension, Result};

fn load_tasks(conn: &Connection) -> Result<Vec<Task>> {
    let mut stmt = conn.prepare("SELECT id, title, done FROM tasks")?;
    let task_iter = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            name: row.get::<_, String>(1)?,
            completed: row.get::<_, bool>(2)?,
        })
    })?;

    let tasks: Vec<Task> = task_iter.filter_map(Result::ok).collect();
    Ok(tasks)
}

pub struct TodoList {
    items: Vec<Task>,
    conn: Connection,
    default_message: String,
}
impl TodoList {
    pub fn new() -> Result<TodoList, Error> {
        let conn = Connection::open("tasks.db")?;
        conn.execute(
            "
            CREATE TABLE IF NOT EXISTS tasks (
                id  INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                done  INTEGER NOT NULL DEFAULT 0
            )",
            [],
        )?;
        Ok(TodoList {
            items: load_tasks(&conn)?,
            conn,
            default_message: String::from(
                "Command List:\n".to_owned()
                    + "\t- ✏️ Enter task name - Create new task.\n"
                    + "\t- ✅ Enter task id - Change task status.\n"
                    + "\t- 🗑️ ':d id' - Delete task by id.\n"
                    + "\t- 📭 ':ca' - Empty task list.\n"
                    + "\t- 👋 ':q' - Quit",
            ),
        })
    }

    pub fn add_task(&mut self, title: &String) -> Result<()> {
        self.conn
            .execute("INSERT INTO tasks (title, done) VALUES (?1, 0)", [&title])?;
        Ok(())
    }
    pub fn delete_task(&mut self, id: u64) -> Result<()> {
        self.conn.execute("DELETE FROM tasks WHERE id = ?1", [id])?;
        Ok(())
    }
    pub fn clear_tasks(&mut self) -> Result<()> {
        self.conn.execute("DELETE FROM tasks", [])?;
        Ok(())
    }
    pub fn toggle_task(&mut self, id: u64) -> Result<()> {
        let mut stmt = self.conn.prepare("SELECT done FROM tasks WHERE id = ?1")?;
        let current: Option<i32> = stmt.query_row([id], |row| row.get(0)).optional()?;
        let Some(current) = current else {
            eprintln!("No task found with id: {}", id);
            return Ok(());
        };
        let new_status = if current == 0 { 1 } else { 0 };

        // updating DB
        self.conn.execute(
            "UPDATE tasks SET done = ?1 WHERE id = ?2",
            [new_status, id as i32],
        )?;

        if let Some(task) = self.items.iter_mut().find(|task| task.id == id) {
            task.completed = new_status != 0;
        }
        Ok(())
    }
    pub fn print(&self) -> Result<()> {
        let mut stmt = self.conn.prepare("SELECT id, title, done FROM tasks")?;
        let task_iter = stmt.query_map([], |row| {
            Ok((
                row.get::<_, i32>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, bool>(2)?,
            ))
        })?;
        let mut found = false;
        for task in task_iter {
            let (id, title, done) = task?;
            found = true;

            println!("{} [{}] {}", id, if done { "X" } else { " " }, title);
        }
        if !found {
            println!("Your task list is empty. 💤");
        }
        Ok(())
    }

    fn print_header(&self, message: &String) {
        clear();
        let _ = self.print();
        line();
        println!("{}", message);
        line();
    }

    pub fn print_home(&mut self, stop_flag: &mut bool) -> Result<()> {
        /* Command Line Input Method
            [1] Defines a commmand for quitting
            [2] Defines a default for empty entries
            [3] Defines the deleting behaviour
            [4] Defines the logic for toggling tasks into done for integer inputs
            [5] Defaults any other text input as new task
        */
        use std::io;
        let mut command = String::new();
        let _ = io::stdin().read_line(&mut command);
        command = (*command.to_lowercase().trim()).to_string();

        let mut message = self.default_message.clone();
        // [1] Setting quitting comand
        if command == ":q" || command == "quit" {
            *stop_flag = true;
        // [2] Defaulting a help message for empty commands
        } else if command == "" {
            message = self.default_message.clone();
        // [3] Setting a delete behaviour for when :d {task id} is entered
        } else if command.starts_with(":d") {
            let parts: Vec<&str> = command.split_whitespace().collect();
            if parts.len() == 2 {
                let arg = parts[1];
                if let Ok(index) = arg.parse::<u64>() {
                    self.delete_task(index)?;
                    message = String::from(format!("Deleted item {i} successfully", i = index));
                } else {
                    message = String::from("Invalid Index: not an integer");
                }
            }
        // [4]
        } else if command == ":ca" {
            let msg = String::from("Do you really wish to clear the list? (y/any)");
            let _ = self.print_header(&msg);
            let mut confirm = String::new();
            let _ = io::stdin().read_line(&mut confirm);
            if confirm.trim().to_lowercase().to_string() == "y" {
                let _ = self.clear_tasks();
            }
        // [5] Toggle tasks with integer input
        } else if let Ok(index) = command.trim().parse::<u64>() {
            self.toggle_task(index)?;
            message = String::from(format!(
                "Status of item {i} changed successfully",
                i = index
            ));
        // [6] Defaults text any other text input as a new task
        } else {
            self.add_task(&command.clone())?;
            message = String::from(format!("Task '{}' successfully added to list!", command))
        }
        self.print_header(&message);
        Ok(())
    }
}
