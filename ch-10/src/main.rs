use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use std::error::Error;
use voca_rs::*;
use hyper_tls::HttpsConnector;
use hyper::Client;

async fn write_to_file(data: &String) -> Result<(), Box<dyn Error>> {
    let mut output_file = File::create("resp-output.txt").await?;
    output_file.write_all(data.as_bytes()).await?;

    Ok(())
}

fn print_to_scren(data: &String) {
    let stripped = strip::strip_tags(data);
    let clean = stripped.replace("\n\n", "");
    println!("{}", clean);
}

#[tokio::main]
async fn main() {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    println!("Hello, world!");
}
