use std::sync::Arc;

use axum::{extract::State, Json};

use crate::{
    internals::ports::app::AddEmployeeApp,
    models::{db_models::AppState, employee_models::EmployeeRequest},
};

use super::base_app::Adaptor;

impl AddEmployeeApp for Adaptor {
    fn add_employee(&self, State(data): State<Arc<AppState>>, request: Json<EmployeeRequest>) {}
}
