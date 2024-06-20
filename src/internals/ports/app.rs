use std::{fmt::Debug, sync::Arc};

use axum::{extract::State, Json};

use crate::models::{db_models::AppState, employee_models::EmployeeRequest};

pub trait EmployeeApp: AddEmployeeApp + Debug + Clone {}

pub trait AddEmployeeApp {
    fn add_employee(&self, data: State<Arc<AppState>>, employee: Json<EmployeeRequest>);
}
