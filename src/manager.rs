use crate::Task;
use std::fs;

pub struct FileManager {
    path: String,
}

impl FileManager {
    pub fn new(p: String) -> Self {
        Self { path: p }
    }

    pub fn get_tasks(&self) -> Vec<Task> {
        let json: String = fs::read_to_string(&self.path).expect("json file should be opened");
        let data: Vec<Task> = serde_json::from_str(&json).expect("json string should be parsed");

        return data;
    }

    pub fn write_tasks(self, tasks: Vec<Task>) {
        let json = serde_json::to_string_pretty(&tasks).expect("tasks should be converted to json");
        fs::write(self.path, json).expect("json should be written to file");
    }
}
