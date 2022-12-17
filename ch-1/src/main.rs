extern crate rand;
use std::{thread, time};

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

    for i in 0..99 {
        for j in 0..99 {
            if rand::random() {
                world[i][j] = 1;
            } else {
                world[i][j] = 0;
            }
        }
    }
}
