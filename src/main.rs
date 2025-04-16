fn header() {
    let n = 50;
    let c = "#".to_string();
    println!("{}", c.repeat(n));
    println!("{i}Todo List{i}", i = " ".to_string().repeat(20));
    println!("{}", c.repeat(n));
}

fn line() {
    let n = 50;
    let c = "-".to_string();
    println!("{}", c.repeat(n));
}

fn clear() {
    std::process::Command::new("clear").status().unwrap();
    header();
}
struct Task {
    id: u64,
    completed: bool,
    name: String,
}
struct TodoList {
    items: Vec<Task>,
    default_message: String,
}
impl TodoList {
    fn new() -> TodoList {
        TodoList {
            items: Vec::new(),
            default_message: String::from(
                "Command List:\n\t- Create New Task: type task name\n\t- Change Status: select task number\n\t- Quit: enter 'q'",
            ),
        }
    }
    fn add_task(&mut self, title: String) {
        let id = self.items.len() as u64 + 1;
        let new_task = Task {
            id,
            completed: false,
            name: title.clone(),
        };
        self.items.push(new_task);
    }
    fn print(&self) {
        // method to print the task list
        if self.items.is_empty() {
            println!("No tasks found!");
        } else {
            // println!(" ID | Title");
            for item in &self.items {
                println!(
                    "{} {} {}",
                    item.id,
                    if item.completed { "[X]" } else { "[ ]" },
                    item.name
                );
            }
        }
    }
    fn print_home(&mut self, stop_flag: &mut bool) {
        use std::io;
        // self.print();
        let mut command = String::new();
        let _ = io::stdin().read_line(&mut command);

        command = (*command.to_lowercase().trim()).to_string();
        let mut message = String::new();
        if command == "q" || command == "quit" {
            *stop_flag = true;
        } else if command == "" {
            message = self.default_message.clone();
        } else if let Ok(index) = command.trim().parse::<usize>() {
            // updating the status of the item and printing
            if 0 < index && index <= self.items.len() {
                let item = &mut self.items[index - 1];
                item.completed = !item.completed;
                message = String::from(format!(
                    "Status of item {i} changed successfully",
                    i = index
                ));
            } else {
                message = String::from(format!(
                    "Index out of bounds (1 to {}). Try again",
                    self.items.len()
                ));
            }
        } else {
            self.add_task(command.clone());
            message = String::from(format!("Task '{}' successfully added to list!", command))
        }
        clear();
        self.print();
        line();
        println!("{}", message);
        line();
    }
}

fn main() {
    let mut todo_list = TodoList::new();
    clear();
    todo_list.print();
    line();
    let mut stop = false;
    while !stop {
        todo_list.print_home(&mut stop);
        //break;
    }
    std::process::Command::new("clear").status().unwrap();
}
