use select::document::Document;
use select::predicate::Name;

static EXAMPLE_PAGE: &str = "https://www.tagesschau.de/ausland/italien-corona-115.html";

async fn main() {
    let res = reqwest::get("https://www.rust-lang.org/en-US/").await.unwrap();

    Document::from_read(res)
        .unwrap()
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| println!("{}", x));
}
