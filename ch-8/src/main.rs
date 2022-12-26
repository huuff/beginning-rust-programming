use std::env;
use std::error::Error;
use crate::database::Database;

mod database;

fn main() -> Result<(), Box<dyn Error>> {
    let database = Database::new()?;

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let command: &str = &args[1];

        match command {
            "add" => database.add_record()?,
            "list" => database.list_records(),
            _ => println!("Didn't send a valid command in"),
        }
    } else {
        println!("Please specify add or list as a command line parameter");
    }

    Ok(())
}
