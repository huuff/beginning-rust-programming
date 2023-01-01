use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    #[command(alias = "DISKS")]
    Disks,
    #[command(alias = "MEMORY")]
    Memory,
    #[command(alias = "PROCESS")]
    Process,
    #[command(alias = "USERS")]
    Users,
    #[command(alias = "NETWORKS")]
    Networks,
}
