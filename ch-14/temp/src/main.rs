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

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("temperatures.txt")?;
    let mut daily_temps: Vec<Temperature> = Vec::new();

    let lines = io::BufReader::new(file).lines();

    for line in lines {
        let line = line?;
        let mut split_line = line.as_str().split(',');
        let left = split_line.next().unwrap();
        let right = split_line.next().unwrap();
        let today = Temperature {
            minimum: left.parse()?,
            maximum: right.parse()?,
        };
        daily_temps.push(today);
    }

    let command = Args::parse().command.unwrap_or(Command::Average);

    match command {
        Command::Average => {
            let avgs = get_average(&daily_temps);
            println!("Average daily low: {}, average daily high: {}", avgs.0, avgs.1);
        }
        Command::Total => {
            let (min_total, max_total) = get_totals(&daily_temps);
            println!("Total daily low: {min_total}, total daily high: {max_total}");
        }
    }


    Ok(())
}
