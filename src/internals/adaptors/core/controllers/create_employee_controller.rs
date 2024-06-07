use std::sync::Arc;

use crate::models::{db::AppState, employee_models::EmployeeRequest};
use axum::{extract::State, response::IntoResponse, Json};
use uuid::Uuid;

use super::base_controller::Adaptor;

impl Adaptor {
    fn create_employee(
        &self,
        State(data): State<Arc<AppState>>,
        Json(body): Json<EmployeeRequest>,
    ) {
        let id = Uuid::new_v4().to_string();
        
        todo!()
    }
}
