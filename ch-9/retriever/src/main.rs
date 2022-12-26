use mongodb::{
    bson::{doc, Bson},
    sync::Client,
};
use std::io;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::with_uri_str("mongodb://localhost:27017")?;
    let collection = client.database("customer_info").collection("people");

    println!("What person would you like to look up?");
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;

    let results = collection.find(doc! { "name": input.trim() }, None)?;

    for result in results {
        if let Some(location) = result?.get("location").and_then(Bson::as_str) {
            println!("location: {}", location);
        } else {
            println!("no location listed");
        }
    }

    Ok(())
}
