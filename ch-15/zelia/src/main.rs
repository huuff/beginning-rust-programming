use std::fs::File;
use std::io::{self, Read, BufRead, BufReader};
use std::error::Error;

struct ChatResponse {
    key: String,
    response: String,
}

fn load_responses(input: Box<dyn BufRead>) -> Result<Vec<ChatResponse>, Box<dyn Error>> {
    let mut response_vector = vec![];

    for line in input.lines() {
        let line = line?;
        let mut split_line = line.as_str().split('\t');
        let r = ChatResponse {
            key: String::from(split_line.next().unwrap()),
            response: String::from(split_line.next().unwrap()),
        };
        response_vector.push(r);
    }

    Ok(response_vector)
}

fn main() -> Result<(), Box<dyn Error>>{
    let filename = "chatresponses.txt";
    let file = BufReader::new(File::open(filename)?);
    let response_vector = load_responses(Box::new(file))?;

    println!("Hi, my name is Zelia, what can I do for you?");

    loop {
        let mut query_input = String::new();
        let mut found = false;

        io::stdin().read_line(&mut query_input)?;
        query_input = query_input.trim().to_string();

        if query_input == "bye" {
            println!("Bye!");
            break;
        }

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

    Ok(())
}
