use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum OperationType {
    Pull,
    Migrate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DatabaseType {
    Postgres,
    MySql,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DbConfig {
    pub db_type: DatabaseType,
    pub host: String,
    pub user: String,
    pub password: String,
    pub database: String,
    pub output_file: String,
    pub type_of_operation: OperationType,
}

impl DbConfig {
    #[warn(dead_code)]
    pub fn new(
        db_type: &str,
        host: &str,
        user: &str,
        password: &str,
        database: &str,
        output_file: &str,
        type_of_operation: &str,
    ) -> DbConfig {
        let config = DbConfig {
            db_type: match db_type.to_lowercase().as_str() {
                "postgres" => DatabaseType::Postgres,
                "mysql" => DatabaseType::MySql,
                _ => DatabaseType::Postgres,
            },
            host: host.to_string(),
            user: user.to_string(),
            password: password.to_string(),
            database: database.to_string(),
            output_file: output_file.to_string(),
            type_of_operation: match type_of_operation.to_lowercase().as_str() {
                "pull" => OperationType::Pull,
                "migrate" => OperationType::Migrate,
                &_ => OperationType::Pull,
            },
        };

        config.save_to_file("config.json").expect("Failed to save to the file");
        config
    }

    pub fn load_from_file(filename: &str) -> Option<Self> {
        let mut file = File::open(filename).ok()?;
        let mut content = String::new();
        file.read_to_string(&mut content).ok()?;
        serde_json::from_str(&content).ok()
    }

    pub fn save_to_file(&self, filename: &str) -> Result<(), std::io::Error> {
        let content = serde_json::to_string(self)?;
        let mut file = File::create(filename)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }
}

impl Default for DbConfig {
    fn default() -> Self {
        DbConfig {
            db_type: DatabaseType::Postgres,
            host: "".to_string(),
            user: "".to_string(),
            password: "".to_string(),
            database: "".to_string(),
            output_file: "".to_string(),
            type_of_operation: OperationType::Pull
        }
    }
}
