use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

struct Task {
    description: String,
    completed: bool,
    created_at: u64,
}

struct TodoList {
    tasks: HashMap<u32, Task>,
    next_id: u32,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList {
            tasks: HashMap::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, description: String) {
        let task = Task {
            description,
            completed: false,
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Failed to get system time")
                .as_secs(),
        };

        self.tasks.insert(self.next_id, task);
        self.next_id += 1;
    }

    fn remove_task(&mut self, task_id: u32) {
        self.tasks.remove(&task_id);
    }

    fn update_task(&mut self, task_id: u32, description: String) {
        if let Some(task) = self.tasks.get_mut(&task_id) {
            task.description = description;
        }
    }

    fn mark_task_as_completed(&mut self, task_id: u32) {
        if let Some(task) = self.tasks.get_mut(&task_id) {
            task.completed = true;
        }
    }

    fn display_task_creation_time(&self, task_id: u32) {
        if let Some(task) = self.tasks.get(&task_id) {
            let created_at = task.created_at;
            println!("Task {} was created at {}", task_id, created_at);
        }
    }

    fn sort_tasks_by_completion(&self) -> Vec<&Task> {
        let mut tasks: Vec<&Task> = self.tasks.values().collect();
        tasks.sort_by(|a, b| a.completed.cmp(&b.completed));
        tasks
    }
}

fn main() {
    let mut todo_list = TodoList::new();

    todo_list.add_task("Buy groceries".to_string());
    todo_list.add_task("Clean the house".to_string());
    todo_list.add_task("Do laundry".to_string());

    todo_list.mark_task_as_completed(2);

    todo_list.update_task(3, "Fold clothes".to_string());

    todo_list.remove_task(1);

    for task in todo_list.sort_tasks_by_completion() {
        println!("Task: {}, Completed: {}", task.description, task.completed);
    }

    todo_list.display_task_creation_time(3);
}
