use crate::config::{DbConfig, DatabaseType};
use sqlx::{mysql::MySqlPoolOptions, postgres::PgPoolOptions, Pool, MySql, Postgres};
use std::error::Error;

pub enum DatabasePool {
    Postgres(Pool<Postgres>),
    MySQL(Pool<MySql>),
}

impl DbConfig {
    pub async fn connection(&self) -> Result<DatabasePool, Box<dyn Error>> {
        match &self.db_type {
            DatabaseType::Postgres => {
                let url = format!(
                    "postgres://{}:{}@{}/{}",
                    self.user, self.password, self.host, self.database
                );
                let pool = PgPoolOptions::new()
                    .max_connections(10)
                    .connect(&url)
                    .await?;
                Ok(DatabasePool::Postgres(pool))
            }
            DatabaseType::MySql => {
                let url = format!(
                    "mysql://{}:{}@{}/{}",
                    self.user, self.password, self.host, self.database
                );
                let pool = MySqlPoolOptions::new()
                    .max_connections(10)
                    .connect(&url)
                    .await?;
                Ok(DatabasePool::MySQL(pool))
            }
        }
    }
}
