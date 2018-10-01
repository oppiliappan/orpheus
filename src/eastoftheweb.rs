extern crate scraper;
extern crate reqwest;

use scraper::{ Html, Selector };
use rand::prelude::*;
use Story;

pub fn get_rand_story() -> Story {
    let mut rng = thread_rng();

    let url: String = "http://www.eastoftheweb.com/short-stories/UBooks/".into();

    let mut response = reqwest::get(&url).unwrap();
    let document = Html::parse_document(&response.text().unwrap());
    let links = Selector::parse("a").unwrap();

    let mut id_vec = vec![];

    for link in document.select(&links) {
        let mut story_id = link.value().attr("href").unwrap();
        id_vec.push(story_id);
    }

    let rand_int = rng.gen_range(0, id_vec.len());
    let story_url = format!(
        "http://www.eastoftheweb.com/cgi-bin/version_printable.pl?story_id={}",
        id_vec.get(rand_int).unwrap()
    );

    let mut story_request = reqwest::get(&story_url).unwrap();
    let story_doc = Html::parse_document(&story_request.text().unwrap());

    // get the author
    let auth_selector = Selector::parse("div.printable_author").unwrap();
    let mut author = String::new();
    for elem in story_doc.select(&auth_selector) {
        author.push_str( & mut elem.text().collect::<String>() );
    }

    // get the title
    let title_selector = Selector::parse("div.printable_title").unwrap();
    let mut title = String::new();
    for elem in story_doc.select(&title_selector) {
        title.push_str( & mut elem.text().collect::<String>() );
    }

    // get the story content
    let mut story: Vec<String> = vec![];
    let content_div_selector = Selector::parse("div.printable_text").unwrap();
    let content_para_selector = Selector::parse("p").unwrap();

    let content_div = story_doc.select(&content_div_selector).next().unwrap();
    for para in content_div.select(&content_para_selector) {
        let para_text = para.text().collect::<String>();
        story.push(para_text);
    }

    Story {
        title: title,
        content: story,
        author: Some(author)
    }

}
