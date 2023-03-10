use std::error::Error;
use clap::Parser;
use crate::database::Database;
use args::{Args, Command};
use dotenv::dotenv;

mod database;
mod args;
mod finding;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let mut database = Database::new().await?;

    let args = Args::parse();

    match args.command {
        Command::Add => database.add_record().await?,
        Command::List => database.list_records().await?,
    }

    Ok(())
}
