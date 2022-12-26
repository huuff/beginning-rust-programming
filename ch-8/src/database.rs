use sqlx::Connection;
use sqlx::sqlite::SqliteConnection;
use std::error::Error;
use std::io::{self, Write};

pub struct Database {
    connection: SqliteConnection,
}

impl Database {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        // TODO: from dotenv
        let mut conn = SqliteConnection::connect("sqlite:stratapp.db").await?;

        sqlx::query!("
          CREATE TABLE IF NOT EXISTS findings(
            findings_ID INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            finding TEXT NOT NULL,
            details TEXT,
            justification TEXT
          )  
        ").execute(&mut conn).await?;

        Ok(Database { connection: conn })
    }

    pub async fn add_record(&mut self) -> Result<(), Box<dyn Error>> {
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

        let title = title.trim();
        let finding = finding.trim();
        let details = details.trim();
        let justification = justification.trim();

        sqlx::query!("
            INSERT INTO findings (title, finding, details, justification) VALUES (?, ?, ?, ?)
        ", title, finding, details, justification).execute(&mut self.connection).await?;

        Ok(())
    }

    pub async fn list_records(&mut self) -> Result<(), Box<dyn Error>> {
        let rows = sqlx::query!("SELECT * FROM findings")
                            .fetch_all(&mut self.connection)
                            .await?;

        // TODO: No unwrapping
        for row in rows {
            println!("-------------------");
            println!("Title = {}", row.title);
            println!("Finding = {}", row.finding);
            println!("Details = {}", row.details.unwrap());
            println!("Justification = {}", row.justification.unwrap());
        }

        Ok(())
    }
}
