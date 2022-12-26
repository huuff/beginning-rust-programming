use std::io::{self, Write};
use std::error::Error;

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

        // TODO: Some way of leaving details and justification as Nones
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

        Ok(Finding {
            title: title.trim().to_string(),
            finding: finding.trim().to_string(),
            details: Some(details.trim().to_string()),
            justification: Some(justification.trim().to_string()),
        })
    }
}
