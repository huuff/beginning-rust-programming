use std::error::Error;
use clap::Parser;
use database::Database;
use args::{Args, Command};

mod database;
mod args;

fn main() -> Result<(), Box<dyn Error>> {
    let database = Database::new()?;

    let args = Args::parse();

    match args.command {
        Command::Add => database.add_record()?,
        Command::List => database.list_records()?,
    }

    Ok(())
}
