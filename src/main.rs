extern crate scraper;
extern crate reqwest;
extern crate rand;
extern crate sciter;

use sciter::dom::Element;

use std::fmt;

pub mod classicshorts;

pub struct Story {
    title: String,
    author: String,
    content: String,
}

impl fmt::Display for Story {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n{}", self.title, self.author);
        write!(f, "{}", self.content);
        write!(f, "\n")
    }
} 

fn main() {
    println!("{}", classicshorts::get_rand_story());
}
