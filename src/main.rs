use select::document::Document;
use select::predicate::Name;

static EXAMPLE_PAGE: &str = "https://www.tagesschau.de/ausland/italien-corona-115.html";

fn main() {
    let res = reqwest::blocking::get(EXAMPLE_PAGE)
        .unwrap()
        .text()
        .unwrap();

    Document::from(res.as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| println!("{}", x));
}
