use std::collections::HashMap;
use chrono::{DateTime, Utc};
use std::time::SystemTime;


struct Task {
    description: String,
    completed: bool,
    created_at: DateTime<Utc>,
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
            created_at: DateTime::from(SystemTime::now()),
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
            let created_at = task.created_at.format("%Y-%m-%d %H:%M:%S").to_string();
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

    todo_list.add_task("CREATE RUST PROJECT".to_string());
    todo_list.add_task("CREATE WORD DOC".to_string());
    todo_list.add_task("GIT PUSH".to_string());

    todo_list.mark_task_as_completed(2);

    todo_list.update_task(3, "PP CREATE".to_string());

    todo_list.remove_task(1);

    for task in todo_list.sort_tasks_by_completion() {
        println!("Task: {}, Completed: {}", task.description, task.completed);
    }

    todo_list.display_task_creation_time(3);
}
