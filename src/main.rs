use select::document::Document;
use select::predicate::Name;

static EXAMPLE_PAGE: &str = "https://www.tagesschau.de/ausland/italien-kliniken-101.html";

fn main() {
    let res = reqwest::blocking::get(EXAMPLE_PAGE)
        .unwrap()
        .text()
        .unwrap();

    Document::from(res.as_str())
        .find(Name("span"))
        .filter(|n| n.attr("class").is_some())
        .filter(|n| n.attr("class").unwrap().contains("stand"))
        .map(|n| n.inner_html())
        .for_each(|x| println!("{}", x));

    Document::from(res.as_str())
        .find(Name("p"))
        .filter(|n| n.attr("class").is_some())
        .filter(|n| n.attr("class").unwrap().contains("autorenzeile"))
        .map(|n| n.inner_html())
        .for_each(|x| println!("{}", x));


    Document::from(res.as_str())
        .find(Name("p"))
        .filter(|n| n.attr("class").is_some())
        .filter(|n| n.attr("class").unwrap().contains("autorenzeile"))
        .map(|n| n.inner_html())
        .for_each(|x| println!("{}", x));
}
