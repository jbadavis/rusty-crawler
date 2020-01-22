extern crate reqwest;
extern crate scraper;

use scraper::{Html, Selector};

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
    let mut links: Vec<_> = vec![String::from("http://jbadavis.github.io")];
    let mut link_store = links.clone();

    println!("Starting at {:?}\n", links[0]);

    for _i in 0..3 {
        let mut links_to_crawl: Vec<_> = vec![];

        for link in links.iter() {
            let mut links_found: Vec<_> = get_links(&link).await;

            println!(
                "Searching {:?}...\nFound {:?} links\n",
                link,
                links_found.len()
            );

            links_to_crawl.append(&mut links_found);
        }

        links = links_to_crawl;
        link_store.append(&mut links.clone());
    }

    println!("{:?}", link_store.len());
}
