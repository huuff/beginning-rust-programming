use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use bufstream::BufStream;
use std::fs;

// lol idk wtf this exercise means
const STRSIZE: i32 = 10;

fn make_directory(param: &str) -> String {
    match fs::create_dir_all(param) {
        Ok(_) => String::from("Success"),
        Err(err) => err.to_string(),
    }
}

fn get_file_list() -> String {
    let mut listing = String::with_capacity(8192);

    for file in fs::read_dir(".").unwrap() {
        let entry = file.unwrap().path().display().to_string();
        listing.push_str(entry.as_str());
    }
    listing
}

fn remove_file(param: &str) -> String {
    match fs::remove_file(param) {
        Ok(_) => String::from("Success"),
        Err(err) => err.to_string(),
    } 
}

fn handle_req(conn: TcpStream) {
    let mut req = String::with_capacity(512);
    let mut response = String::with_capacity(4096);
    let mut reader = BufStream::new(&conn);

    match reader.write(b"> ") {
        Ok(_) => (),
        Err(err) => println!("Received an error on write: {}", err)
    };
    reader.flush().unwrap();
    println!("Sent prompt to client");

    let size = reader.read_line(&mut req);
    println!("Received a command");
    if size.unwrap() > 0 {
        let mut params = req.split_whitespace();
        let command = params.next().unwrap();
        println!("Running command {}", command);
        match command {
            "flist" => response = get_file_list(),
            "md" => response = make_directory(params.next().unwrap()),
            "rf" => response = remove_file(params.next().unwrap()),
            _ => response = String::from("Unacceptable command")
        }
        match reader.write(&response.into_bytes()) {
            Ok(_) => (),
            Err(err) => println!("Received an error on write {}", err),
        }
        reader.flush().unwrap();
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:3333")?;

    for stream in listener.incoming() {
        let stream = stream?;
        println!("Received connection from {}",stream.peer_addr()?.to_string());
        handle_req(stream);
    }

    Ok(())
}
