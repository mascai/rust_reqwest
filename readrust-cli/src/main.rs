use clap::Parser;
use reqwest;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;


#[derive(Debug, Deserialize, Serialize)]
struct Author {
    name: String,
    url: String,
}


#[derive(Debug, Deserialize, Serialize)]
struct FeedItem {
    id: String,
    title: String,
    content_text: String,
    url: String,
    date_published: String,
    author: Author,
}


#[derive(Debug, Deserialize, Serialize)]
struct Feed {
    version: String,
    title: String,
    home_page_url: String,
    feed_url: String,
    description: String,
    author: Author,
    items: Vec<FeedItem>,
}


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Total number of posts
    #[arg(short, long)]
    count: i32,

    // Number of posts to display
    #[arg(short, long)]
    number: i32
}

pub static URL: &str = "http://readrust.net/rust2018/feed.json";


fn get_feed() -> Feed {
    let client = reqwest::blocking::Client::new();
    let mut request = client.get(URL);
    let mut resp = request.send().unwrap();

    assert!(resp.status().is_success());
    let json = resp.text().unwrap();
    return serde_json::from_str(&json).unwrap();
}


fn print_count(feed: &Feed) {
    println!("Number of posts: {}", feed.items.len());
}


fn main() {
    let args = Args::parse();

    let feed = get_feed();

    print_count(&feed);
}