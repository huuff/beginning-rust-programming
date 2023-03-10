use std::io::{self, Write};
use std::error::Error;
use serde::{Serialize, Deserialize};

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct Finding {
    pub title: String,
    pub finding: String,
    pub details: Option<String>,
    pub justification: Option<String>,
}

impl Finding {
    pub fn from_stdin() -> Result<Self, Box<dyn Error>> {
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

        let details = {
            let details = details.trim();
            if details.is_empty() {
                None
            } else {
                Some(details.to_string())
            }
        };

        let justification = {
            let justification = justification.trim();
            if justification.is_empty() {
                None
            } else {
                Some(justification.to_string())
            }
        };

        Ok(Finding {
            title: title.trim().to_string(),
            finding: finding.trim().to_string(),
            details, justification,
        })
    }
}
