use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    pub input: Option<Input>,
}

#[derive(Subcommand)]
pub enum Input {
    Stdin,
    File {
        file: String
    }
}
