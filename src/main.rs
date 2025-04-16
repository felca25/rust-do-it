struct Task {
    id: u64,
    completed: bool,
    name: String,
}
struct TodoList {
    items: Vec<Task>,
}
impl TodoList {
    fn new() -> TodoList {
        TodoList { items: Vec::new() }
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
            println!(" ID | Title");
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
}

fn main() {
    let mut todo_list = TodoList::new();
    todo_list.print();
    todo_list.add_task(String::from("Testing tasks"));
    todo_list.print();
}
