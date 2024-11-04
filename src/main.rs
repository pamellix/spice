mod cli;
mod config;
mod database;

use crate::database::{backup, restore};
use clap::Parser;

#[tokio::main]
async fn main() {
    let cli = cli::Cli::parse();

    let config = match cli.command {
        cli::Commands::Migrate(migrate_cmd) => migrate_cmd.load_or_promt(),
    };
    
    match config.connection().await {
        Ok(pool) => {
            match config.type_of_operation {
                config::OperationType::Pull => {
                    if let Err(e) = backup::backup_database(&pool, &config.output_file).await {
                        eprintln!("Backup failed: {}", e);
                    }
                }
                config::OperationType::Migrate => {
                    if let Err(e) = restore::restore_database(&pool, &config.output_file).await {
                        eprintln!("Restore failed: {}", e);
                    }
                }
            }
        }
        Err(e) => eprintln!("Database connection failed: {}", e),
    }
}
