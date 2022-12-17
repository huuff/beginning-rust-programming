extern crate rand;
extern crate termion;
use std::{env, thread, time};
use std::fs::File;
use std::io::{BufRead, BufReader};
use termion::clear;
use termion::color;

fn census(_world: [[u8; 100]; 100]) -> u16 {
    let mut count = 0;

    for i in 0..99 {
        for j in 0..99 {
            if _world[i][j] == 1 {
                count += 1;
            }
        }
    }
    count
}

fn generation(_world: [[u8; 100]; 100]) -> [[u8; 100]; 100] {
    let mut new_world = [[0u8; 100]; 100];

    for i in 0..99 {
        for j in 0..99 {
            let mut count = 0;
            if i > 0 {
                count += _world[i-1][j];
            }
            if i > 0 && j > 0 {
                count += _world[i-1][j+1];
            }
            if i > 0 && j < 99 {
                count += _world[i-1][j+1];
            }
            if i < 99 {
                count += _world[i+1][j];
            }
            if i < 99 && j < 99 {
                count += _world[i+1][j+1];
            }
            if j > 0 {
                count += _world[i][j-1];
            }
            if j < 99 {
                count += _world[i][j+1];
            }

            new_world[i][j] = 0;

            if (count < 2) && (_world[i][j] == 1) {
                new_world[i][j] = 0;
            }
            if _world[i][j] == 1 && (count == 2 || count == 3) {
                new_world[i][j] = 1;
            }
            if (_world[i][j] == 0) && (count == 3) {
                new_world[i][j] = 1;
            }
        }
    }
    new_world
}

fn main() {
    let mut world = [[0u8; 100]; 100];
    let mut generations = 0;

    let args : Vec<String> = env::args().collect();

    if args.len() < 2 {
        for i in 0..99 {
            for j in 0..99 {
                if rand::random() {
                    world[i][j] = 1;
                } else {
                    world[i][j] = 0;
                }
            }
        }
    } else {
        // TODO: Can't I get it from args above?
        let filename = env::args().nth(1).unwrap();
        world = populate_from_file(filename);
    }

    println!("Population at generation {} is {}", generations, census(world));

    for _gens in 0..100 {
        let temp = generation(world);
        world = temp;
        generations += 1;
        println!("{}", clear::All);
        display_world(world);
        println!(
            "{blue}Population at generation {g} is {c}",
            blue = color::Fg(color::Blue),
            g = generations,
            c = census(world),
        );
        thread::sleep(time::Duration::from_secs(2));
    }

}

fn populate_from_file(filename: String) -> [[u8; 100]; 100] {
    let mut new_world = [[0u8; 100]; 100];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut pairs: Vec<(usize, usize)> = Vec::new();
    
    for (index, line) in reader.lines().enumerate() {
        let l = line.unwrap();
        let mut words = l.split_whitespace();
        let left = words.next().unwrap();
        let right = words.next().unwrap();
        pairs.push((left.parse::<usize>().unwrap(), right.parse::<usize>().unwrap()));
    }

    for i in 0..99 {
        for j in 0..99 {
            new_world[i][j] = 0;
        }
    }

    for (x, y) in pairs {
        new_world[x][y] = 1;
    }
    new_world
}

fn display_world(world: [[u8; 100]; 100]) {
    for i in 0..99 {
        for j in 0..99 {
            if world[i][j] == 1 {
                print!("{red}*", red = color::Fg(color::Red));
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}
