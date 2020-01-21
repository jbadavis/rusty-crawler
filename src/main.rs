extern crate reqwest;
extern crate scraper;

use scraper::{Html, Selector};

async fn get_body() -> Result<String, Box<dyn std::error::Error>> {
    let body = reqwest::get("https://jbadavis.github.io")
        .await?
        .text()
        .await?;

    Ok(body)
}

fn get_links(body: &str) -> Vec<String> {
    Html::parse_document(body)
        .select(&Selector::parse("a").unwrap())
        .map(|elm| String::from(elm.value().attr("href").unwrap()))
        .collect()
}

#[tokio::main]
async fn main() {
    let body = get_body().await.unwrap();

    let links = get_links(&body);

    println!("{:?}", links);
}
