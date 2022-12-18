extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs::{File, OpenOptions};

#[derive(Serialize, Deserialize)]
struct Dvd {
    name: String,
    year: u16,
    cast: String,
    length: u16,
}

fn json_from_str(raw: &str) -> Dvd {
   serde_json::from_str(raw).unwrap()
}

fn str_from_json(advd: &Dvd) -> String {
    serde_json::to_string(advd).unwrap()
}

fn dvds_to_file(f: &String, d: &Vec<Dvd>) {
    let file = OpenOptions::new().append(true).open(f).unwrap();
    serde_json::to_writer(file, &d).unwrap();
}

fn dvds_from_file(f: &String) -> Vec<Dvd> {
    let file = File::open(f).unwrap();
    let deserialized_json: Vec<Dvd> = serde_json::from_reader(file).unwrap();
    deserialized_json
}

fn main() {
    let la_la_land = r#"{
        "name": "La La Land",
        "year": 2016,
        "cast": "Emma Stone, Ryan Gosling",
        "length": 126
    }"#;

    let cannibal_holocaust = r#"{
        "name": "Cannibal Holocaust",
        "year": 1980,
        "cast": "Perry Pirkanen, Francesca Ciardi",
        "length": 95
    }"#;

    let mut dvds = vec![json_from_str(la_la_land), json_from_str(cannibal_holocaust)];

    let filename = String::from("file.json");
    dvds_to_file(&filename, &dvds);

    dvds = dvds_from_file(&filename);

    for dvd in dvds {
        println!("{}", str_from_json(&dvd));
    }
}
