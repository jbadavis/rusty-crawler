extern crate reqwest;
extern crate scraper;

use futures::stream::StreamExt;
use scraper::{Html, Selector};
use std::time::Instant;

async fn get_doc(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let doc = reqwest::get(url).await?.text().await?;

    Ok(doc)
}

fn parse_links(doc: &str) -> Vec<String> {
    Html::parse_document(doc)
        .select(&Selector::parse("a").unwrap())
        .map(|elm| {
            if let Some(url) = elm.value().attr("href") {
                String::from(url)
            } else {
                String::new()
            }
        })
        .filter(|s| s.starts_with("http"))
        .collect()
}

async fn get_links(link: &str) -> Vec<String> {
    match get_doc(link).await {
        Ok(doc) => parse_links(&doc),
        Err(_) => vec![],
    }
}

#[tokio::main]
async fn main() {
    let mut links: Vec<_> = vec![String::from("https://jbadavis.github.io")];
    let mut links_to_crawl: Vec<_> = links.clone();

    println!("\nStarting from {}", links[0]);

    loop {
        let now = Instant::now();

        let r = futures::stream::iter(
            links_to_crawl
                .iter()
                .map(|link| async move { get_links(&link).await }),
        )
        .buffer_unordered(50)
        .collect::<Vec<_>>()
        .await;

        links_to_crawl = r.iter().flatten().cloned().collect::<Vec<String>>();
        links.append(&mut links_to_crawl.clone());

        println!(
            " - {} links found in {}ms\n   Total found {}\n",
            links_to_crawl.len(),
            now.elapsed().as_millis(),
            links.len(),
        );
    }
}
