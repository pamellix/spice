use crate::commands::dbconfig::{DbConfig, OperationType};
use clap::Parser;
use std::path::Path;

#[derive(Parser)]
#[command(name = "spice", about = "Utility for managing database operations")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Parser)]
pub enum Commands {
    Migrate(MigrateCommand),
}

#[derive(Parser)]
pub struct MigrateCommand {
    #[arg(short = 't', default_value = "pull", help = "Select a type for a operation")]
    pub operation_type: String,

    #[arg(help = "Specify the database type")]
    pub db_type: Option<String>,

    #[arg(help = "Specify the host")]
    pub host: Option<String>,

    #[arg(short = 'u', help = "Specify the user")]
    pub user: Option<String>,

    #[arg(short = 'p', help = "Specify the password")]
    pub password: Option<String>,

    #[arg(short = 'd', help = "Specify the database")]
    pub database: Option<String>,

    #[arg(short = 'o', help = "Specify the output file for the backup")]
    pub output_file: String
}

impl MigrateCommand {
    pub fn load_or_promt(self) -> DbConfig {
        let config_path = "config.json";

        let mut config = if Path::new(config_path).exists() {
            DbConfig::load_from_file(&config_path).unwrap_or_else(|| {
                println!("Failed to load config. Defaulting to manual input");
                DbConfig::default()
            })
        } else {
            DbConfig::default()
        };

        if let Some(db_type) = self.db_type {
            config.db_type = db_type;
        };

        if let Some(host) = self.host {
            config.host = host;
        };

        if let Some(user) = self.user {
            config.user = user;
        };

        if let Some(password) = self.password {
            config.password = password;
        };

        if let Some(database) = self.database {
            config.database = database;
        };

        config.output_file = self.output_file.clone();
        config.type_of_operation = match self.operation_type.as_str() {
            "migrate" => OperationType::Migrate,
            _ => OperationType::Pull, 
        };

        config.save_to_file(config_path).expect("Failed to save configuration");
        config

    }
}

pub fn parse_args() -> DbConfig {
    let cli = Cli::parse();

    match cli.command {
        Commands::Migrate(migrate_cmd) => {
            let db = migrate_cmd.load_or_promt();

            db.show_test_result();
            db
        }
    }
}