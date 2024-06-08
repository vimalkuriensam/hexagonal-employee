use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use clap::Parser;
use dotenv::from_filename_iter;

use crate::{internals::ports::core::ConfigPort, models::base_models::{Config, EnvironmentType}};

#[derive(Debug, Parser, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Adaptor {
    #[arg(long, default_value = "development")]
    environment: EnvironmentType,
    #[arg(skip)]
    config: Arc<Mutex<Config>>,
}

pub fn initialize() -> Box<Adaptor> {
    let config = Config {
        env: HashMap::new(),
        database: None,
    };
    let mut args = Adaptor::parse();
    args.config = Arc::new(Mutex::new(config));
    Box::new(args)
}
impl ConfigPort for Adaptor {
    fn load_environment(&mut self) -> Result<Box<Adaptor>, String> {
        if matches!(self.environment, EnvironmentType::Production) {
        } else {
            let path = std::env::current_dir().unwrap();
            let file_dir = format!("{}/environment/{}.env", path.display(), self.environment);
            println!("{}", file_dir);
            match from_filename_iter(file_dir) {
                Ok(iter_val) => {
                    let mut config = self.config.lock().unwrap();
                    for item in iter_val {
                        let (key, value) = item.unwrap();
                        config.env.insert(key, value);
                    }
                }
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }
        Ok(Box::new(self.clone()))
    }

    fn get_config(&self) -> Config {
        self.config.lock().unwrap().clone()
    }
}
