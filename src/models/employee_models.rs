use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Employee {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub age: u8,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct EmployeeRequest {
    #[validate(length(min = 3))]
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[validate(length(min = 3))]
    #[serde(rename = "lastName")]
    pub last_name: String,
    #[validate(range(min = 18, max = 70))]
    pub age: u8,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeResponse {}
