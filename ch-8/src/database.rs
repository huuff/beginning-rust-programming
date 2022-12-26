use sqlite::{Connection, State};
use std::error::Error;
use std::io::{self, Write};

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let conn = Connection::open("stratapp.db")?;

        conn.execute("
          CREATE TABLE IF NOT EXISTS findings(
            findings_ID INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            finding TEXT NOT NULL,
            details TEXT,
            justification TEXT
          )  
        ")?;

        Ok(Database { connection: conn })
    }

    pub fn add_record(&self) -> Result<(), Box<dyn Error>> {
        let mut title = String::new();
        let mut finding = String::new();
        let mut details = String::new();
        let mut justification = String::new();

        print!("Title: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut title)?;
        print!("Finding: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut finding)?;
        print!("Details: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut details)?;
        print!("Justification: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut justification)?;

        let command_string = format!(r#"
            INSERT INTO FINDINGS (title, finding, details, justification)
            VALUES ("{}", "{}", "{}", "{}")
        "#, title.trim(), finding.trim(), details.trim(), justification.trim());
        self.connection.execute(&command_string)?;

        Ok(())
    }

    pub fn list_records(&self) -> Result<(), Box<dyn Error>> {
        let mut statement = self.connection
            .prepare("SELECT * FROM findings")?;

        while let State::Row = statement.next()? {
            println!("-------------------");
            println!("Title = {}", statement.read::<String, _>(1)?);
            println!("Finding = {}", statement.read::<String, _>(2)?);
            println!("Details = {}", statement.read::<String, _>(3)?);
            println!("Justification = {}", statement.read::<String, _>(4)?);
        }

        Ok(())
    }
}
