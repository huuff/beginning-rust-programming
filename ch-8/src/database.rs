use sqlx::{Connection,Row};
use sqlx::sqlite::SqliteConnection;
use std::error::Error;
use std::io::{self, Write};
use futures::TryStreamExt;

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

        let command_string = format!(r#"
            INSERT INTO FINDINGS (title, finding, details, justification)
            VALUES ("{}", "{}", "{}", "{}")
        "#, title.trim(), finding.trim(), details.trim(), justification.trim());

        sqlx::query(command_string.as_str()).execute(&mut self.connection).await?;

        Ok(())
    }

    pub async fn list_records(&mut self) -> Result<(), Box<dyn Error>> {
        let mut rows = sqlx::query("SELECT * FROM findings")
                            .fetch(&mut self.connection)
                            ;

        while let Some(row) = rows.try_next().await? {
            println!("-------------------");
            println!("Title = {}", row.try_get::<String, _>("title")?);
            println!("Finding = {}", row.try_get::<String, _>("finding")?);
            println!("Details = {}", row.try_get::<String, _>("details")?);
            println!("Justification = {}", row.try_get::<String, _>("justification")?);
        }

        Ok(())
    }
}
