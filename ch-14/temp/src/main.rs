mod args;

use std::fs::File;
use std::io::{self, BufRead};
use std::error::Error;
use args::{Args, Command};
use clap::Parser;

#[derive(Copy, Clone)]
struct Temperature {
    minimum: f32,
    maximum: f32,
}

fn get_totals(temps: &Vec<Temperature>) -> (f32, f32) {
    let mut min_total = 0_f32;
    let mut max_total = 0_f32;

    for t in temps {
        min_total = min_total + t.minimum;
        max_total = max_total + t.maximum;
    }

    (min_total, max_total)
}

fn get_average(temps: &Vec<Temperature>) -> (f32, f32) {
    let (min_total, max_total) = get_totals(temps);
    let count = temps.len();

    (min_total / count as f32, max_total / count as f32)
}

fn partial_min<T: PartialOrd>(x: T, y: T) -> Option<T> {
    let cmp = x.partial_cmp(&y)?;

    match cmp {
        std::cmp::Ordering::Less => Some(x),
        std::cmp::Ordering::Greater => Some(y),
        std::cmp::Ordering::Equal => Some(x),
    }
}

fn partial_max<T: PartialOrd>(x: T, y: T) -> Option<T> {
    let cmp = x.partial_cmp(&y)?;

    match cmp {
        std::cmp::Ordering::Less => Some(y),
        std::cmp::Ordering::Greater => Some(x),
        std::cmp::Ordering::Equal => Some(x),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("temperatures.txt")?;
    let mut daily_temps: Vec<Temperature> = Vec::new();

    let lines = io::BufReader::new(file).lines();

    for line in lines {
        let line = line?;
        let mut split_line = line.as_str().split(',');
        let left: f32 = split_line.next().unwrap().parse()?;
        let right: f32 = split_line.next().unwrap().parse()?;
        let today = Temperature {
            minimum: partial_min(left, right).unwrap(),
            maximum: partial_max(left, right).unwrap(),
        };
        daily_temps.push(today);
    }

    let command = Args::parse().command.unwrap_or(Command::Average);

    match command {
        Command::Average => {
            let (min_avg, max_avg) = get_average(&daily_temps);
            println!("Average daily low: {min_avg}, average daily high: {max_avg}");
        },
        Command::Total => {
            let (min_total, max_total) = get_totals(&daily_temps);
            println!("Total daily low: {min_total}, total daily high: {max_total}");
        },
        Command::All => {
            for Temperature { minimum, maximum } in daily_temps {
                println!("Minimum: {minimum}, maximum: {maximum}");
            }
        }
    };


    Ok(())
}
