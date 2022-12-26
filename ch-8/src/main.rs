use std::error::Error;
use clap::Parser;
use crate::database::Database;
use args::{Args, Command};

mod database;
mod args;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut database = Database::new().await?;

    let args = Args::parse();

    match args.command {
        Command::Add => database.add_record().await?,
        Command::List => database.list_records().await?,
    }

    Ok(())
}
