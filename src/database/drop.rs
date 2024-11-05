use crate::database::connection::DatabasePool;
use sqlx::Row;
use std::error::Error;

#[warn(dead_code)]
pub async fn drop_all_tables(pool: &DatabasePool) -> Result<(), Box<dyn Error>> {
    match pool {
        DatabasePool::Postgres(pg_pool) => {
            let mut conn = pg_pool.acquire().await?;
            let tables = sqlx::query("SELECT table_name FROM information_schema.tables WHERE table_schema = 'public'")
                .fetch_all(&mut conn).await?;
            for table in tables {
                let table_name: String = table.try_get("table_name")?;
                let drop_query = format!("DROP TABLE IF EXISTS {} CASCADE;", table_name);
                sqlx::query(&drop_query).execute(&mut conn).await?;
            }
        }
        DatabasePool::MySQL(mysql_pool) => {
            let mut conn = mysql_pool.acquire().await?;
            let tables = sqlx::query("SELECT table_name FROM information_schema.tables WHERE table_schema = DATABASE()")
                .fetch_all(&mut conn).await?;
            for table in tables {
                let table_name: String = table.try_get("table_name")?;
                let drop_query = format!("DROP TABLE IF EXISTS {};", table_name);
                sqlx::query(&drop_query).execute(&mut conn).await?;
            }
        }
    }
    println!("All tables dropped successfully");
    Ok(())
}
