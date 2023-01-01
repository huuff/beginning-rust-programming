mod args;

use clap::Parser;
use args::{Args, Input};
use std::{error::Error, io::{BufRead, BufReader}, fs::File};

fn main() -> Result<(), Box<dyn Error>> {
    let input = Args::parse().input.unwrap_or(Input::Stdin);
    let input: Box<dyn BufRead> = match input {
        Input::File { file } => {
            Box::new(BufReader::new(File::open(file)?))
        },
        Input::Stdin => {
            Box::new(BufReader::new(std::io::stdin()))
        },
    };

    let mut vec = vec![];

    for line in input.lines() {
        let line = line?;
        let num: i32 = line.parse()?;
        vec.push(num);
    }

    vec.sort_by(|x, y| y.cmp(&x));

    println!("The numbers you inputted are:");
    while let Some(num) = vec.pop() {
       println!("{}", num);
    }

    Ok(())
}
