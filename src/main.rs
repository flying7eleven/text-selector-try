use select::document::Document;
use select::predicate::Name;

static EXAMPLE_PAGE: &str = "https://www.heise.de/security/meldung/Achtung-Sicherheitspatch-gegen-kritische-SMBv3-Luecke-jetzt-verfuegbar-4681993.html";

fn main() {
    let res = reqwest::blocking::get(EXAMPLE_PAGE)
        .unwrap()
        .text()
        .unwrap();

    let headline = Document::from(res.as_str())
        .find(Name("h1"))
        .filter(|n| n.attr("itemprop").is_some())
        .filter(|n| n.attr("itemprop").unwrap().contains("headline"))
        .map(|n| n.inner_html().trim().to_string())
        .last()
        .unwrap();

    let author = Document::from(res.as_str())
        .find(Name("li"))
        .filter(|n| n.attr("itemprop").is_some())
        .filter(|n| n.attr("itemprop").unwrap().contains("author"))
        .map(|n| n.inner_html().trim().to_string())
        .last()
        .unwrap();

    let published = Document::from(res.as_str())
        .find(Name("time"))
        .filter(|n| n.attr("datetime").is_some())
        .map(|n| n.attr("datetime").unwrap().to_string())
        .last()
        .unwrap();

    let article = Document::from(res.as_str())
        .find(Name("div"))
        .filter(|n| n.attr("class").is_some())
        .filter(|n| n.attr("class").unwrap().contains("article-content"))
        .map(|n| n.inner_html().trim().to_string())
        .last()
        .unwrap();

    println!("Headline: {}", headline);
    println!("Author: {}", author);
    println!("Published: {}", published);
    println!("Article text: {}", article);
}
