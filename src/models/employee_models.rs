use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Employee {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub age: u8,
    pub title: String,
}
