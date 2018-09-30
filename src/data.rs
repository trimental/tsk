use std::fs::OpenOptions;
use std::path::PathBuf;

use task;

use xdg;

use serde_json;

use chrono::Local;

pub struct TskData {
    pub location: PathBuf,
    pub tasks: Vec<task::Task>,
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
        };
        tsk_data.read_tasks();

        tsk_data
    }

    pub fn avaliable_id(&mut self) -> usize {
        let mut ids = Vec::new();
        for task in &self.tasks {
            ids.push(task.id);
        }
        for i in 1.. {
            if !ids.contains(&Some(i)) {
                return i;
            }
        }
        unreachable!()
    }

    pub fn add_task(&mut self, task: task::Task) {
        self.tasks.push(task);
        self.write_tasks();
    }

    pub fn delete_task(&mut self, id: usize) {
        let index = self.to_index(id).expect("Not a valid id");
        self.tasks.remove(index);
        self.write_tasks()
    }

    pub fn delete_all(&mut self) {
        self.tasks.clear();
        self.write_tasks()
    }

    pub fn complete_task(&mut self, id: usize) {
        let index = self.to_index(id).expect("Not a valid id");
        {
            let task = &mut self.tasks[index];
            task.comp_time = Some(Local::now());
            task.id = None;
        }
        self.write_tasks()
    }

    pub fn complete_all(&mut self) {
        for task in &mut self.tasks {
            task.comp_time = Some(Local::now());
            task.id = None;
        }
        self.write_tasks()
    }

    pub fn write_tasks(&self) {
        let data_file = OpenOptions::new()
            .create(self.location.exists())
            .write(true)
            .truncate(true)
            .open(self.location.clone())
            .unwrap();
        serde_json::to_writer(&data_file, &self.tasks).unwrap()
    }

    pub fn read_tasks(&mut self) {
        let data_file = OpenOptions::new()
            .create(self.location.exists())
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

    pub fn get_task(&mut self, id: usize) -> Option<task::Task> {
        for task in &mut self.tasks {
            if task.id? == id {
                return Some(task.clone());
            }
        }
        None
    }

    pub fn to_index(&self, id: usize) -> Option<usize> {
        for (i, task) in self.tasks.iter().enumerate() {
            if task.id? == id {
                return Some(i);
            }
        }
        None
    }
}
