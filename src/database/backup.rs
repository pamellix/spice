use crate::database::connection::DatabasePool;
use sqlx::{Row, Column};
use std::fs::File;
use std::io::Write;
use std::error::Error;

pub async fn backup_database(pool: &DatabasePool, output_file: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(output_file)?;

    match pool {
        DatabasePool::Postgres(pg_pool) => {
            let mut conn = pg_pool.acquire().await?;
            let tables = sqlx::query("SELECT table_name FROM information_schema.tables WHERE table_schema = 'public'")
                .fetch_all(&mut conn)
                .await?;

            for table in tables {
                let table_name: String = table.try_get("table_name")?;
                writeln!(file, "\n-- TABLE --: {}\n", table_name)?;
                
                let create_table_stmt = sqlx::query(&format!(
                    "SELECT 'CREATE TABLE {} (' || string_agg(column_name || ' ' || data_type, ', ') || ');' as create_statement \
                    FROM information_schema.columns WHERE table_name = '{}'",
                    table_name, table_name
                ))
                .fetch_one(&mut conn)
                .await?;
                
                let create_statement: String = create_table_stmt.try_get("create_statement")?;
                writeln!(file, "{};", create_statement)?;

                let rows = sqlx::query(&format!("SELECT * FROM {}", table_name))
                    .fetch_all(&mut conn)
                    .await?;

                for row in rows {
                    let mut column_names = String::new();
                    let mut values = String::new();

                    for (i, column) in row.columns().iter().enumerate() {
                        let column_name = column.name();
                        let value: Result<String, _> = row.try_get(i);
                        let value = match value {
                            Ok(val) => format!("'{}'", val.replace("'", "''")),
                            Err(_) => "NULL".to_string(),
                        };

                        column_names.push_str(&format!("{}, ", column_name));
                        values.push_str(&format!("{}, ", value));
                    }

                    let column_names = column_names.trim_end_matches(", ");
                    let values = values.trim_end_matches(", ");
                    writeln!(file, "INSERT INTO {} ({}) VALUES ({});", table_name, column_names, values)?;
                }
            }
        }
        DatabasePool::MySQL(mysql_pool) => {
            let mut conn = mysql_pool.acquire().await?;
            let tables = sqlx::query("SELECT table_name FROM information_schema.tables WHERE table_schema = DATABASE()")
                .fetch_all(&mut conn)
                .await?;

            for table in tables {
                let table_name: String = table.try_get("table_name")?;
                writeln!(file, "\n-- TABLE --: {}\n", table_name)?;

                let create_table_stmt = sqlx::query(&format!(
                    "SHOW CREATE TABLE {}",
                    table_name
                ))
                .fetch_one(&mut conn)
                .await?;
                
                let create_statement: String = create_table_stmt.try_get("Create Table")?;
                writeln!(file, "{};", create_statement)?;

                let rows = sqlx::query(&format!("SELECT * FROM {}", table_name))
                    .fetch_all(&mut conn)
                    .await?;

                for row in rows {
                    let mut column_names = String::new();
                    let mut values = String::new();

                    for (i, column) in row.columns().iter().enumerate() {
                        let column_name = column.name();
                        let value: Result<String, _> = row.try_get(i);
                        let value = match value {
                            Ok(val) => format!("'{}'", val.replace("'", "''")),
                            Err(_) => "NULL".to_string(),
                        };

                        column_names.push_str(&format!("{}, ", column_name));
                        values.push_str(&format!("{}, ", value));
                    }

                    let column_names = column_names.trim_end_matches(", ");
                    let values = values.trim_end_matches(", ");
                    writeln!(file, "INSERT INTO {} ({}) VALUES ({});", table_name, column_names, values)?;
                }
            }
        }
    }

    println!("Backup completed successfully in {}", output_file);
    Ok(())
}
