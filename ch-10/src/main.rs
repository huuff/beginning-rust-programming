use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use std::error::Error;

async fn write_to_file(data: &String) -> Result<(), Box<dyn Error>> {
    let mut output_file = File::create("resp-output.txt").await?;
    output_file.write_all(data.as_bytes()).await?;

    Ok(())
}

fn main() {
    println!("Hello, world!");
}
