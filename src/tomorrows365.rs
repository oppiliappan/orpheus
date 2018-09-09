extern crate scraper;
extern crate reqwest;
extern crate selectors;

use scraper::{ Html, Selector };
use rand::prelude::*;
use self::selectors::Element;
use Story;

pub fn get_rand_story() -> Story {
    let story_list_url = "https://365tomorrows.com/flashes-of-fiction/";

    let mut response = reqwest::get(story_list_url).unwrap();
    let document = Html::parse_document(&response.text().unwrap());
    let story_selector = Selector::parse("a.more-link").unwrap();

    let mut url_vec = vec![];
    for link in document.select(&story_selector) {
        let mut url = link.value().attr("href").unwrap();
        url_vec.push(url);
    }
    // choose a random story from the listing
    let mut rng = thread_rng();
    let rand_int: usize = rng.gen_range(0, url_vec.len());
    let story_url = url_vec.get(rand_int).unwrap();


    let mut story_request = reqwest::get(*story_url).unwrap();
    let story_document = Html::parse_document(&story_request.text().unwrap());

    // get the story content
    let mut story = String::new();
    let content_div_selector = Selector::parse("div.entry-content").unwrap();
    let content_para_selector = Selector::parse("p").unwrap();

    let content_div = story_document.select(&content_div_selector).next().unwrap();
    for para in content_div.select(&content_para_selector) {
        story.push_str(& mut para.text().collect::<String>());
        story.push_str("\n\n");
    }

    // get the story title
    let mut title = String::new();
    let title_selector = Selector::parse("h1.entry-title").unwrap();
    for elem in story_document.select(&title_selector) {
        title.push_str( & mut elem.text().collect::<String>() );
    }

    Story {
        title: title,
        content: story
    }
}
