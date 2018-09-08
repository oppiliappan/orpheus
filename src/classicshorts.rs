extern crate scraper;
extern crate reqwest;

use scraper::{ Html, Selector };
use rand::prelude::*;
use Story;

pub fn get_rand_story() -> Story {
    let mut result = String::new();
    let base_url: String = String::from("http://www.classicshorts.com/abc/");
    let mut rng = thread_rng();

    let category = match rng.gen_range(0,4) {
        0 => "a-d.html",
        1 => "e-h.html",
        2 => "i-m.html",
        3 => "n-s.html",
        _ => "t-z.html"
    };

    let net_url = base_url + category;

    let mut response = reqwest::get(&net_url).unwrap();
    let document = Html::parse_document(&response.text().unwrap());
    let st_selector = Selector::parse(r#"div[class=storylisting]"#).unwrap();

    // get the urls in the listing
    let mut url_vec = vec![];
    for listing in document.select(&st_selector) {
        let mut url = listing.value().attr("onclick").unwrap();
        url_vec.push(&url[20..( url.len() - 7 )]);
    }

    // choose a random story from the listing
    let rand_int: usize = rng.gen_range(0, url_vec.len());
    let story_title = url_vec.get(rand_int).unwrap();
    result.push_str(story_title);


    let mut story_response = reqwest::get(&format!("http://www.classicshorts.com/stories/{}.html", result)).unwrap();
    let story_doc = Html::parse_document(&story_response.text().unwrap());

    // get the story content
    let story_para_selector = Selector::parse(r#"div[class=StoryPara]"#).unwrap();
    let mut story = String::new();
    for div in story_doc.select(&story_para_selector) {
        // story.append(& mut div.text().map(|&x| x.to_string()).collect::<Vec<_>>());
        story.push_str(& mut div.text().collect::<String>());
        story.push_str("\n");
    }

    // get the story title
    let title_selector = Selector::parse(r#"div[onClick=gotoSpecificBib\(\)]"#).unwrap();
    let mut title = String::new();
    for div in story_doc.select(&title_selector) {
        title.push_str(& mut div.text().collect::<String>());
    }

    // get the story author
    let author_selector = Selector::parse(r#"div[onClick=gotoSpecificBio\(\)]"#).unwrap();
    let mut author = String::new();
    for div in story_doc.select(&author_selector) {
        author.push_str(& mut div.text().collect::<String>());
    }

    Story {
        title: title,
        author: author,
        content: story
    }
}
