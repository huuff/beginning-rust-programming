#![feature(decl_macro)]
#[macro_use] extern crate rocket;

use rocket::Data;
use std::{fs::{self, File}, io::Write};

#[get("/")]
fn index() -> &'static str {
    "Welcome to your very own server, but there is nothing at the main branch\n"
}

#[get("/bacon")]
fn bacon() -> String {
    let bacon_contents = fs::read_to_string("bacon.txt")
        .expect("Unable to open file");

    format!("{}\n", bacon_contents)
}

#[post("/upload", format = "plain", data = "<data>")]
fn upload(data: Data) -> Result<String, std::io::Error> {
    let mut msg: Vec<u8> = Vec::new();
    data.stream_to(&mut msg)?;

    let mut file = File::create("/tmp/data.txt")?;
    file.write_all(&msg)?;
    let msg = String::from_utf8(msg).unwrap();
    println!("{}", msg);

    Ok(format!("Wrote {} bytes", msg.len())) 
}

#[get("/greetz/<name>/<age>")]
fn greetz(name: String, age: u8) -> String {
    if name == "Wuutz" {
        String::from("Hello Wuutz, you the best!")
    } else {
        format!("Greetz, {} year old named {}!", age, name)
    }
}

#[get("/ofage/<name>/<age>")]
fn ofage(name: String, age: u8) -> String {
    if age > 18 {
        format!("Welcome, {}, your age {} means you can view this content!", name, age)
    } else {
        format!("Sorry, {}, you are not the right age, since you are only {} years old", name, age)
    }
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index,greetz,upload,bacon,ofage])
        .launch();
}
