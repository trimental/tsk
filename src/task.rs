use chrono::{DateTime, Local};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Priority {
    High,
    Medium,
    Low,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: Option<usize>,
    pub title: String,
    pub description: String,
    pub priority: Option<Priority>,
    pub comp_time: Option<DateTime<Local>>,
}

impl Task {
    pub fn new<S: Into<String>>(id: usize, title: S, description: S) -> Task {
        Task {
            id: Some(id),
            title: title.into(),
            description: description.into(),
            priority: None,
            comp_time: None,
        }
    }
}
