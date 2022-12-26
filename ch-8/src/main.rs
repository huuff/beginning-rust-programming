use sqlite;
use sqlite::Connection;
use std::env;
use std::error::Error;

mod dbfuncs;

fn main() -> Result<(), Box<dyn Error>> {
    let conn = Connection::open("stratapp.db")?;

    conn.execute("
        CREATE TABLE IF NOT EXISTS findings(
            findings_ID INTEGER PRIMARY KEY
            title TEXT NOT NULL
            findings TEXT NOT NULL
            details TEXT
            justification TEXT
        )    
    ")?;

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let command: &str = &args[1];

        match command {
            "add" => dbfuncs::add_record(&conn)?,
            "list" => dbfuncs::list_records(&conn),
            _ => println!("Didn't send a valid command in"),
        }
    } else {
        println!("Please specify add or list as a command line parameter");
    }

    Ok(())
}
