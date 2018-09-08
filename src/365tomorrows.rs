extern crate scraper;
extern crate reqwest;

use scraper::{ Html, Selector };
use rand::prelude::*;
use Story;

pub fn get_rand_story() -> Story {
    let story_list_url = "https://365tomorrows.com/flashes-of-fiction/"

    let mut response = reqwest::get(&story_list_url).unwrap();
    let document = Html::parse_document(&response.text().unwrap());
    let story_selector = Selector::parse(r#"a[class=more-link]"#).unwrap();

    let mut url_vec = vec![];
    for link in document.select(&story_selector) {
        let mut url = listing.value().attr("href").unwrap();
        url_vec.push(url);
    }

    // choose a random story from the listing
    let mut rng = thread_rng();
    let rand_int: usize = rng.gen_range(0, url_vec.len());
    let story_url = url_vec.get(rand_int).unwrap();

    // fetch the story
    let story_request = reqwest::get(&story_url).unwrap();
    let story_document = Html::parse_document(&story_request).unwrap();

    // get the story title
    let title_selector = Selector::parse(r#"h1[class=entry-title]"#).unwrap();
}
