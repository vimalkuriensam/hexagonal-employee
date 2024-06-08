use std::{collections::HashMap, fmt, str::FromStr, sync::Arc};

use sqlx::PgPool;

#[derive(Debug, Default, Clone)]
pub struct Config {
    pub env: HashMap<String, String>,
    pub database: Option<Arc<PgPool>>,
}

#[derive(Debug, Clone)]
pub enum EnvironmentType {
    Development,
    Production,
    Testing,
}

impl FromStr for EnvironmentType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "development" => Ok(EnvironmentType::Development),
            "production" => Ok(EnvironmentType::Production),
            "testing" => Ok(EnvironmentType::Testing),
            _ => Err(format!("'{}' is not a valid environment type", s)),
        }
    }
}

impl fmt::Display for EnvironmentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            EnvironmentType::Development => "development",
            EnvironmentType::Production => "production",
            EnvironmentType::Testing => "testing",
        };
        write!(f, "{}", s)
    }
}