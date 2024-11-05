mod cli;
mod config;
mod database;

use crate::database::{backup, restore};
use clap::Parser;
use log::{info, error};

#[tokio::main]
async fn main() {
    env_logger::init();

    let cli = cli::Cli::parse();

    let config = match cli.command {
        cli::Commands::Migrate(migrate_cmd) => migrate_cmd.load_or_promt(),
    };
    
    match config.connection().await {
        Ok(pool) => {
            info!("Connection established");

            match config.type_of_operation {
                config::OperationType::Pull => {
                    info!("Starting backup process");
                    if let Err(e) = backup::backup_database(&pool, &config.output_file).await {
                        error!("Backup failed: {}", e);
                    } else {
                        info!("Backup completed");
                    }
                }
                config::OperationType::Migrate => {
                    info!("Starting restore process");
                    if let Err(e) = restore::restore_database(&pool, &config.output_file).await {
                        error!("Restore failed: {}", e);
                    } else {
                        info!("Restore completed");
                    }
                }
            }
        }
        Err(e) => error!("Database connection failed: {}", e),
    }
}
