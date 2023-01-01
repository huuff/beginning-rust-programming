use std::vec::Vec;
use std::error::Error;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct DVD {
    title: String,
    year: u32,
    minutes: u32,
}

impl DVD {
    pub fn new(title: String, year: u32, minutes: u32) -> Self {
        DVD { title, year, minutes }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut movies = vec![
        DVD::new("Buckaroo Banzai Across the 8th Dimension".to_string(), 1984, 90),
        DVD::new("Captain America".to_string(), 2011, 80),
        DVD::new("Stargate".to_string(), 1994, 120),
        DVD::new("When Harry Met Sally".to_string(), 1989, 85),
        DVD::new("Kiss Kiss Bang Band".to_string(), 2005, 92),
        DVD::new("The Dark Knight".to_string(), 2008, 200),
        DVD::new("Boys Night Out".to_string(), 1962, 75),
        DVD::new("The Glass Bottom Boat".to_string(), 1966, 70),
    ];

    movies.sort_by(|a, b| b.minutes.cmp(&a.minutes));

    while let Some(movie) = movies.pop() {
        println!("{:?}", movie);
    }

    Ok(())
}
