extern crate scraper;
extern crate reqwest;
extern crate rand;
extern crate sciter;

use sciter::dom::Element;

use std::fmt;

pub mod classicshorts;
pub mod tomorrows365;

pub struct Story {
    title: String,
    content: String,
}

impl fmt::Display for Story {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n{}", self.title, self.content)
    }
} 

fn main() {
    println!("{}", tomorrows365::get_rand_story());
}
