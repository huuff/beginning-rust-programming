use sqlx::Connection;
use sqlx::sqlite::SqliteConnection;
use std::error::Error;
use std::env;
use crate::finding::Finding;

pub struct Database {
    connection: SqliteConnection,
}

impl Database {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let db_url = env::var("DATABASE_URL")?;
        let mut conn = SqliteConnection::connect(db_url.as_str()).await?;

        sqlx::query!("
          CREATE TABLE IF NOT EXISTS findings(
            findings_id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            finding TEXT NOT NULL,
            details TEXT,
            justification TEXT
          )  
        ").execute(&mut conn).await?;

        Ok(Database { connection: conn })
    }

    pub async fn add_record(&mut self) -> Result<(), Box<dyn Error>> {
        let finding = Finding::from_stdin()?;

        sqlx::query!("
            INSERT INTO findings (title, finding, details, justification) VALUES (?, ?, ?, ?)
        ", finding.title, finding.finding, finding.details, finding.justification).execute(&mut self.connection).await?;

        Ok(())
    }

    pub async fn list_records(&mut self) -> Result<(), Box<dyn Error>> {
        let rows = sqlx::query_as::<_, Finding>("SELECT title, finding, details, justification FROM findings")
            .fetch_all(&mut self.connection)
            .await?;

        for row in rows {
            println!("-------------------");
            println!("{:?}", row);
        }

        Ok(())
    }
}
