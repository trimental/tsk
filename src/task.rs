#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Priority {
    High,
    Medium,
    Low,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: usize,
    pub title: String,
    pub description: String,
    pub priority: Option<Priority>,
}

impl Task {
    pub fn new<S: Into<String>>(id: usize, title: S, description: S) -> Task {
        Task {
            id,
            title: title.into(),
            description: description.into(),
            priority: None,
        }
    }

    pub fn set_priority(&mut self, priority: Option<Priority>) {
        self.priority = priority
    }
}
