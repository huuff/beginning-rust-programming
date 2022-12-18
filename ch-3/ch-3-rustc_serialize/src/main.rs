extern crate rustc_serialize;
use rustc_serialize::json::{self, ToJson, Json};

#[derive(RustcEncodable)]
struct Dvd {
    name: String,
    year: u16,
    cast: String,
    length: u16,
    director: String,
}

impl ToJson for Dvd {
    fn to_json(&self) -> Json {
        Json::String(format!("{}+{}+{}+{}i", self.name, self.year, self.cast, self.length))
    }
}

fn convert_to_json(advd: &Dvd) -> String {
    json::encode(advd).unwrap()
}

fn main() {
    let a = Dvd {
        name: String::from("Four Weddings and a Funeral"),
        year: 1994,
        cast: String::from("Hugh Grant"),
        length: 117,
        director: String::from("Mike Newell"),
    };

    let encoded = convert_to_json(&a);

    println!("{}", encoded);
}
