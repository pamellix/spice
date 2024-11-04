use crate::database::connection::DatabasePool;
use std::fs;
use std::error::Error;

pub async fn restore_database(pool: &DatabasePool, input_file: &str) -> Result<(), Box<dyn Error>> {
    let sql_script = fs::read_to_string(input_file)?;
    let commands = sql_script.split(";");

    match pool {
        DatabasePool::Postgres(pg_pool) => {
            let mut conn = pg_pool.acquire().await?;
            for command in commands {
                let trimmed_command = command.trim();
                if !trimmed_command.is_empty() {
                    sqlx::query(trimmed_command).execute(&mut conn).await?;
                }
            }
        }
        DatabasePool::MySQL(mysql_pool) => {
            let mut conn = mysql_pool.acquire().await?;
            for command in commands {
                let trimmed_command = command.trim();
                if !trimmed_command.is_empty() {
                    sqlx::query(trimmed_command).execute(&mut conn).await?;
                }
            }
        }
    }

    println!("Database restored successfully from {}", input_file);
    Ok(())
}
