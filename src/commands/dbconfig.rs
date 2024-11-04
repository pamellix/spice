use std::{fs, io::Read};
use std::fs::File;
use sqlx::{postgres::PgPoolOptions, Column, Row};
use std::io::Write;
use serde::{Serialize, Deserialize};
use sqlx::postgres::PgRow;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum OperationType {
    Pull,
    Migrate
}

#[derive(Debug, Serialize, Deserialize, Clone )]
pub struct DbConfig {
    pub db_type: String,
    pub host: String,
    pub user: String,
    pub password: String,
    pub database: String,
    pub output_file: String,
    pub type_of_operation: OperationType
}

impl DbConfig {
    pub fn new(db_type: &str, host: &str, user: &str, password: &str, database: &str, output_file: &str, type_of_operation: &str) -> DbConfig {
        let config = DbConfig {
            db_type: db_type.to_string(),
            host: host.to_string(),
            user: user.to_string(),
            password: password.to_string(),
            database: database.to_string(),
            output_file: output_file.to_string(),
            type_of_operation: match type_of_operation {
                "pull" => OperationType::Pull,
                "migrate" => OperationType::Migrate,
                &_ => OperationType::Pull
            }
        };

        config.save_to_file("config.json").expect("Failed to load to the file");
        config
    }

    pub fn get_type_of_operation(&self) -> OperationType {
        println!("{:?}", self.type_of_operation);
        self.type_of_operation
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
        file.write_all(content.as_bytes())
    }

    pub async fn connection(&self) -> Result<sqlx::Pool<sqlx::Postgres>, sqlx::Error> {
        println!("Connecting to the database");

        let config = DbConfig::load_from_file("config.json").unwrap_or_else(|| {
            println!("Configuration not found. Saving current configuration");
            self.clone()
        });

        let url = format!(
            "postgresql://{}:{}@{}/{}",
            config.user, config.password, config.host, config.database
        );

        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&url)
            .await?;
        println!("Connection successful!");

        Ok(pool)
    }

    pub async fn drop_all_tables(&self) -> Result<(), sqlx::Error> {
        let pool = self.connection().await?;
        let mut conn = pool.acquire().await?;

        let tables: Vec<PgRow> = sqlx::query("SELECT table_name FROM information_schema.tables WHERE table_schema = 'public'")
            .fetch_all(&mut conn).await?;

        for table in tables {
            let table_name: String = table.try_get("table_name")?;
            let drop_query = &format!("DROP TABLE IF EXISTS {} CASCADE;", table_name);
            sqlx::query(drop_query).fetch_one(&mut conn).await?;
        }

        println!("All tables dropped successfully");

        Ok(())
    }

    pub async fn restore_database(&self) -> Result<(), sqlx::Error> {
        let pool = &self.connection().await?;
        let sql_script = fs::read_to_string(&self.output_file)?;
        let mut conn = pool.acquire().await?;

        let commands = sql_script.split(";");

        for command in commands {
            let trimmed_command = command.trim();

            if !trimmed_command.is_empty() {
                sqlx::query(trimmed_command).execute(&mut conn).await?;
                println!("COMMAND EXECUTED SUCCESSFUL, COMMAND: {}", trimmed_command);
            }
        }

        println!("Database restored successfully from {}", &self.output_file);

        Ok(())
    }

    pub async fn backup_database(&self) -> Result<(), sqlx::Error> {
        let pool = self.connection().await?;
        let mut conn = pool.acquire().await?;

        let output_file = &self.output_file;
        let mut file = File::create(output_file)?;

        let tables = sqlx::query("SELECT table_name FROM information_schema.tables WHERE table_schema = 'public'")
            .fetch_all(&mut conn)
            .await?;

        for table in tables {
            let table_name: String = table.try_get("table_name")?;

            writeln!(file, "\n-- TABLE --: {}\n", table_name)?;
            let create_table_stmt = sqlx::query(&format!("SELECT 'CREATE TABLE {} (' || string_agg(column_name || ' ' || data_type, ', ') || ');' as create_statement FROM information_schema.columns WHERE table_name = '{}'", table_name, table_name))
                .fetch_one(&mut conn)
                .await?;
            let create_statement: String = create_table_stmt.try_get("create_statement")?;
            writeln!(file, "{};", create_statement)?;


           let rows = sqlx::query(&format!("SELECT * FROM {}", table_name))
               .fetch_all(&mut conn)
               .await?;

            for row in rows {
                let mut column_names: String = String::new();
                let mut values = String::new();

                for (i, column) in row.columns().iter().enumerate() {
                    let column_name = column.name();
                    let value: Result<String, _> = row.try_get(i);
                    let value = match value {
                        Ok(val) => {format!("'{}'", val.replace("'", "''"))}
                        Err(_) => {"NULL".to_string()}
                    };

                    column_names.push_str(&format!("{}, ", column_name));
                    values.push_str(&format!("{}, ", value));
                }

                let column_names = column_names.trim_end_matches(", ");
                let values = values.trim_end_matches(", ");
                writeln!(file, "INSERT INTO {} ({}) VALUES ({});", table_name, column_names, values)?;
            }
        }

        println!("Backup completed successfully in {}", output_file);
        
        Ok(())
    }

    pub fn show_test_result(&self) {
        println!(
            "Database works on {} for a user {} with host {}",
            self.database, self.user, self.host
        );
    }
}

impl Default for DbConfig {
    fn default() -> Self {
        DbConfig {
            db_type: "postgres".to_string(),
            host: "".to_string(),
            user: "".to_string(),
            password: "".to_string(),
            database: "".to_string(),
            output_file: "".to_string(),
            type_of_operation: OperationType::Pull
        }
    }
}
