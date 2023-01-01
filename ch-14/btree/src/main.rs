mod args;

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error::Error;
use args::{Args, Command};
use clap::Parser;

fn fill_tree() -> Result<BTreeMap<String, i32>, Box<dyn Error>> {
    let file = File::open("values.txt")?;

    let mut movie_entries: BTreeMap<String, i32> = BTreeMap::new();

    let lines = BufReader::new(file).lines();

    for line in lines {
        let line = line?;
        let mut split_line = line.as_str().split(',');
        let left = split_line.next().unwrap();
        let right = split_line.next().unwrap();
        let year = right.parse::<i32>().unwrap();
        movie_entries.insert(String::from(left), year);
    }

    Ok(movie_entries)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut movies: BTreeMap<String, i32> = fill_tree()?;


    let command = Args::parse().command;

    if let Some(command) = command {
        match command {
            Command::Add => {
                let mut input = String::with_capacity(128);
                std::io::stdin().read_line(&mut input)?;
                let mut split_input = input.as_str().trim().split(',');
                let left = split_input.next().unwrap();
                let right = split_input.next().unwrap();
                let year = right.parse::<i32>().unwrap();
                movies.insert(String::from(left), year);
            },
            Command::Query { q } => {
                let year = movies.get(&q);

                if let Some(year) = year {
                    println!("{q}: {}", year.to_string());
                } else {
                    eprintln!("Movie {q} not found");
                }
            },
        }
    } else {

    }

    println!("We have {} movies: ", movies.len());
    for (movie, year) in &movies {
        println!("{}: {}", movie, year);
    }

    //match movies.get("Captain America") {
    //Some(year) => println!("{}", year),
    //None => println!("Unable to find that movie"),
    //}

    Ok(())
}
