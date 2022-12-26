use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
   #[arg(short, long, default_value_t = false)]
    pub print: bool,

    #[arg(short, long, default_value_t = false)]
    pub write: bool,

    pub hostname: String,
}
