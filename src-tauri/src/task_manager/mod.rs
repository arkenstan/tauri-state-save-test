use std::{collections::HashMap, sync::Mutex};

use serde::{Deserialize, Serialize};

use self::tasks::TaskMap;

pub mod tasks;

#[derive(Serialize, Deserialize)]
pub struct Manager {
    pub tasks: Mutex<TaskMap>,
}

impl Manager {
    pub fn new() -> Self {
        let mut tasks = TaskMap(HashMap::new());
        tasks.init();

        if let Some(loaded_tasks) = tasks.load() {
            tasks = loaded_tasks;
        }

        Self {
            tasks: Mutex::new(tasks),
        }
    }

    pub fn save(&self) {
        let _ = &self.tasks.lock().unwrap().save();
    }
}
