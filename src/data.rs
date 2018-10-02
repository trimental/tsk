use std::fs::OpenOptions;
use std::path::PathBuf;

use task;

use xdg;

use serde_json;

use chrono::Local;

pub struct TskData {
    pub location: PathBuf,
    pub tasks: Vec<task::Task>,
    pub order: Vec<usize>,
}

impl TskData {
    pub fn new() -> TskData {
        let xdg_dirs = xdg::BaseDirectories::with_prefix("tsk").unwrap();
        let location = xdg_dirs.place_data_file("tasks").unwrap();

        OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(location.clone())
            .unwrap();
        let data_len = location.metadata().unwrap().len() as usize;

        let mut tsk_data = TskData {
            location,
            tasks: Vec::with_capacity(data_len),
            order: Vec::new(),
        };
        tsk_data.read_tasks();
        tsk_data.order();

        tsk_data
    }

    pub fn order(&mut self) {
        let mut order = Vec::new();

        // Sort by priority
        let mut high = Vec::new();
        let mut medium = Vec::new();
        let mut low = Vec::new();
        for (i, task) in self
            .tasks
            .iter()
            .filter(|t| t.comp_time.is_none())
            .enumerate()
        {
            match task.priority {
                Some(task::Priority::High) => high.push(i),
                Some(task::Priority::Medium) | None => medium.push(i),
                Some(task::Priority::Low) => low.push(i),
            }
        }

        // Sort by creation date
        high.sort_by(|a, b| {
            self.tasks[*b]
                .creation_time
                .cmp(&self.tasks[*a].creation_time)
        });
        medium.sort_by(|a, b| {
            self.tasks[*b]
                .creation_time
                .cmp(&self.tasks[*a].creation_time)
        });
        low.sort_by(|a, b| {
            self.tasks[*b]
                .creation_time
                .cmp(&self.tasks[*a].creation_time)
        });
        order.append(&mut high);
        order.append(&mut medium);
        order.append(&mut low);
        self.order = order;
    }

    pub fn add_task(&mut self, task: task::Task) {
        self.tasks.push(task);
        self.order();
        self.write_tasks();
    }

    pub fn delete_task(&mut self, index: usize) {
        let index = *self.order.get(index).expect("Not a valid index");
        self.tasks.remove(index);
        self.order();
        self.write_tasks()
    }

    pub fn delete_all(&mut self) {
        self.tasks.clear();
        self.order();
        self.write_tasks()
    }

    pub fn complete_task(&mut self, index: usize) {
        let index = *self.order.get(index).expect("Not a valid index");
        {
            let task = &mut self.tasks[index];
            task.comp_time = Some(Local::now());
        }
        self.order();
        self.write_tasks()
    }

    pub fn complete_all(&mut self) {
        for task in &mut self.tasks {
            task.comp_time = Some(Local::now());
        }
        self.order();
        self.write_tasks()
    }

    pub fn write_tasks(&self) {
        let data_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(self.location.clone())
            .unwrap();
        serde_json::to_writer(&data_file, &self.tasks).unwrap()
    }

    pub fn read_tasks(&mut self) {
        let data_file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(self.location.clone())
            .unwrap();

        self.tasks = if data_file.metadata().unwrap().len() != 0 {
            serde_json::from_reader(&data_file).unwrap()
        } else {
            Vec::new()
        };
    }

    pub fn get_task(&mut self, index: usize) -> Option<task::Task> {
        let index = *self.order.get(index)?;
        Some(self.tasks[index].clone())
    }
}
