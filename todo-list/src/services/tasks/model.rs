use serde::{Serialize, Deserialize};

#[derive(PartialEq, Serialize, Deserialize, Debug)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub status: TaskStatus,
}