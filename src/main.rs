extern crate scraper;
extern crate rand;
extern crate sciter;

use sciter::dom::Element;
use rand::prelude::*;

use std::fmt;

pub mod classicshorts;
pub mod tomorrows365;

pub struct Story {
    title: String,
    content: Vec<String>,
}

impl fmt::Display for Story {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n", self.title);
        write!(f, "\n")
    }
} 

fn main() {
    let mut rng = thread_rng();
    let rand_int: usize = rng.gen_range(0, 2);
    let story = match rand_int {
        0 => classicshorts::get_rand_story(),
        _ => tomorrows365::get_rand_story(),
    };


    let mut frame = sciter::Window::new();
    frame.load_file("ui_templates/story.html");
    let root = Element::from_window(frame.get_hwnd()).unwrap();
    if let Some(mut ele) = root.find_first("#title").unwrap() {
        match ele.set_text(&story.title) {
            _ => {},
        };
    }

    if let Some(mut ele) = root.find_first("#content").unwrap() {
        for para in story.content {
            let p = Element::with_text("p", &para).unwrap();
            match ele.append(&p) {
                _ => {},
            };
        }
    }

    frame.run_app();
}
