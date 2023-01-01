use std::fs::File;
use std::io::{self, Read, BufRead};
use std::error::Error;

struct ChatResponse {
    key: String,
    response: String,
}

fn main() -> Result<(), Box<dyn Error>>{
    let filename = "chatresponses.txt";
    let mut response_vector = vec![];
    
    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();
    for line in lines {
        let line = line?;
        let mut split_line = line.as_str().split('\t');
        let r = ChatResponse {
            key: String::from(split_line.next().unwrap()),
            response: String::from(split_line.next().unwrap()),
        };
        response_vector.push(r);
    }

    println!("Hi, my name is Zelia, what can I do for you?");

    loop {
        let mut query_input = String::new();
        let mut found = false;

        io::stdin().read_line(&mut query_input)?;
        query_input = query_input.trim().to_string();

        for resp in &response_vector {
            if query_input.contains(resp.key.as_str()) {
                found = true;
                println!("{}", resp.response);
                break;
            }
        }

        if !found {
            println!("I'm not sure what you are saying");
        }
    }
}
