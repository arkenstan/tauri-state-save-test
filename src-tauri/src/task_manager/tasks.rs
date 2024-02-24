use std::{collections::HashMap, fs::File, path::Path};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    task_id: String,
    created_at: String,
}

impl Task {
    pub fn new() -> Self {
        let task_id = Uuid::new_v4().to_string();
        let dt: DateTime<Utc> = Utc::now();
        let created_at = format!("{}", dt);
        Self {
            task_id,
            created_at,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskMap(pub HashMap<String, Task>);

impl TaskMap {
    fn file_path(&self) -> String {
        return String::from("/home/arkenstan/test.dat");
    }

    pub fn init(&self) {
        let test_file_path = self.file_path();

        let file_path = Path::new(&test_file_path);

        if !file_path.exists() {
            let file = File::create(file_path).unwrap();
            serde_json::to_writer(file, self).unwrap();
        }
    }

    pub fn add_task(&mut self, task: Task) {
        self.0.insert(task.task_id.clone(), task);
    }

    pub fn load(&self) -> Option<TaskMap> {
        let test_file_path = self.file_path();

        let file_path = Path::new(&test_file_path);

        if file_path.exists() {
            let file = File::options().read(true).open(file_path).unwrap();
            serde_json::from_reader(file).unwrap()
        } else {
            None
        }
    }

    pub fn save(&self) {
        let test_file_path = self.file_path();

        let file_path = Path::new(&test_file_path);

        if file_path.exists() {
            let file = File::options()
                .write(true)
                .truncate(true)
                .open(file_path)
                .unwrap();
            serde_json::to_writer(file, &self).unwrap()
        }
    }
}
