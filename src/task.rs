use chrono::{DateTime, Local};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Priority {
    High,
    Medium,
    Low,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub title: String,
    pub description: String,
    pub priority: Option<Priority>,
    pub creation_time: DateTime<Local>,
    pub comp_time: Option<DateTime<Local>>,
    pub tags: Vec<String>,
}

impl Task {
    pub fn new<S: Into<String>>(title: S, description: S) -> Task {
        Task {
            title: title.into(),
            description: description.into(),
            priority: None,
            creation_time: Local::now(),
            comp_time: None,
            tags: Vec::new(),
        }
    }
}
