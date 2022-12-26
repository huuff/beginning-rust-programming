use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use std::error::Error;
use voca_rs::*;
use hyper_tls::HttpsConnector;
use hyper::{Client, body::HttpBody as _};
use std::env;

async fn write_to_file(data: &String) -> Result<(), Box<dyn Error>> {
    let mut output_file = File::create("resp-output.txt").await?;
    output_file.write_all(data.as_bytes()).await?;

    Ok(())
}

fn print_to_screen(data: &String) {
    let stripped = strip::strip_tags(data);
    let clean = stripped.replace("\n\n", "");
    println!("{}", clean);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let mut hostname = String::new();
    let mut write_file = false;
    let mut print_file = false;
    let mut body = String::new();

    let args: Vec<String> = env::args().collect();
    for position in 1..(args.len()) {
        if position == args.len()-1 {
            hostname = String::from(&args[position]);
        }

        match args[position].as_str() {
            "-w" => write_file = true,
            "-p" => print_file = true,
            _ => (),
        }
    }

    let mut res = client.get(hostname.parse()?).await?;
    println!("Headers:\n{:#?}", res.headers());

    while let Some(chunk) = res.body_mut().data().await {
        let chunk = chunk?;
        body.push_str(&(String::from_utf8_lossy(&chunk)));
    }

    if write_file {
        write_to_file(&body).await?;
    }

    if print_file {
        print_to_screen(&body);
    }

    Ok(())
}
