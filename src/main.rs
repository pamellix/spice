use crate::commands::dbconfig::OperationType;

mod cli;
mod commands;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let backup_config = cli::parse_args();

    match backup_config.get_type_of_operation() {
        OperationType::Pull => backup_config.backup_database().await?,
        OperationType::Migrate => backup_config.restore_database().await?,
    }

    println!("PLS SUBSCRRIBE TO ME <3");

    Ok(())
}
