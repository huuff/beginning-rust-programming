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

fn find_response<'a>(response_vector: &'a Vec<ChatResponse>, input: &str) -> Option<&'a str> {
    for resp in response_vector {
        if input.contains(resp.key.as_str()) {
            return Some(resp.response.as_str());
        }
    }

    None
}

fn main() -> Result<(), Box<dyn Error>>{
    let filename = "chatresponses.txt";
    let file = BufReader::new(File::open(filename)?);
    let response_vector = load_responses(Box::new(file))?;

    println!("Hi, my name is Zelia, what can I do for you?");

    loop {
        let mut query_input = String::new();

        io::stdin().read_line(&mut query_input)?;
        query_input = query_input.trim().to_string();

        if query_input == "bye" {
            println!("Bye!");
            break;
        }

        if let Some(response) = find_response(&response_vector, &query_input) {
            println!("{}", response);
        } else {
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

    #[test]
    fn finds_response() {
        // ARRANGE
        let response_vector = vec![
            ChatResponse {
                key: String::from("key"),
                response: String::from("response")
            }
        ];

        // ACT
        let response = find_response(&response_vector, "this contains key");

        // ASSERT
        assert!(response.is_some());
        assert_eq!(response.unwrap(), "response");
    }

    #[test]
    fn doesnt_find_response() {
        // ARRANGE
        let response_vector = vec![
            ChatResponse {
                key: String::from("key"),
                response: String::from("response")
            }
        ];

        // ACT
        let response = find_response(&response_vector, "this doesnt contsain it");

        // ASSERT
        assert!(response.is_none());
    }
}
