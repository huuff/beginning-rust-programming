use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;
use std::error::Error;

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
    let movies: BTreeMap<String, i32> = fill_tree()?;

    println!("We have {} movies", movies.len());

    match movies.get("Captain America") {
        Some(year) => println!("{}", year),
        None => println!("Unable to find that movie"),
    }

    for (movie, year) in &movies {
        println!("{}: {}", movie, year);
    }

    let mut movie_vec = Vec::from_iter(movies);
    movie_vec.sort_by(|&(_, a), &(_,b)| a.cmp(&b));

    println!("{:?}", movie_vec);

    Ok(())
}
