use std::io::prelude::*;
use std::net::TcpStream;
use std::env;
use std::io;
use std::thread;
use std::time::Duration;
use regex::Regex;

fn validate_input(input: &String) -> bool {
    let mut valid: bool = false;
    let mut params = input.split_whitespace();
    let command = params.next().unwrap();
    match command {
        "flist" => valid = true,
        "md" => valid = true,
        _ => valid = false,
    }
    valid
}

fn handle_input(mut server_stream: TcpStream) {
    let mut recv_string = [0; 4096];
    
    let mut keep_going: bool = true;
    let regex = Regex::new(r"^[eE][xX][iI][tT]$").unwrap();

    let mut size = server_stream.read(&mut recv_string);
    println!("{}", String::from_utf8_lossy(&recv_string));

    while keep_going {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                input = input.trim().to_string();
                if regex.is_match(input.as_str()) {
                    keep_going = false;
                } else {
                    if validate_input(&input) {
                        match server_stream.write(&input.as_bytes()) {
                            Ok(_n) => { () },
                            Err(_e) => {
                                panic!("Unable to write to server");
                            }
                        }
                    } else {
                        println!("Not a valid command");
                    }
                    size = server_stream.read(&mut recv_string);
                    println!("{}", String::from_utf8_lossy(&recv_string));
                }
            },
            Err(err) => {
                println!("Error: {:?}", err);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let server_string = &args[1];

    match TcpStream::connect(server_string) {
        Ok(server_stream) => {
            println!("Successfully connected to {}", server_string);
            thread::sleep(Duration::from_secs(1));
            handle_input(server_stream);
        },
        Err(e) => {
            panic!("Unable to connect to {}", server_string);
        }
    }
}
