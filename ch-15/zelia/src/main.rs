use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::error::Error;

#[derive(Eq, PartialEq, Debug)]
struct ChatResponse {
    key: String,
    response: String,
}

fn load_responses(input: Box<dyn BufRead>) -> Result<Vec<ChatResponse>, Box<dyn Error>> {
    let mut response_vector = vec![];

    for line in input.lines() {
        let line = line?;
        let mut split_line = line.as_str().split(',');
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

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn correctly_load_responses() -> Result<(), Box<dyn Error>>{
        // ARRANGE
        let input = BufReader::new(indoc! {r#"
            firstkey,firstresponse
            secondkey,secondresponse
        "#}.trim().as_bytes());

        // ACT
        let response_vector = load_responses(Box::new(input))?;

        // ASSERT
        assert_eq!(response_vector[0], ChatResponse {
            key: String::from("firstkey"),
            response: String::from("firstresponse"),
        });
        
        Ok(())
    }
}
