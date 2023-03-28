use reqwest::Client;
use select::document::Document;
use select::predicate::{Attr, Name, Predicate};
use std::env;

async fn search_dark_web(query: &str) -> Result<(), reqwest::Error> {
    let search_url = format!("https://ahmia.fi/search/?q={}", query);
    let client = Client::new();
    let response = client.get(&search_url).send().await?;
    let html = response.text().await?;

    let document = Document::from(html.as_str());
    let mut found_match = false;

    for (i, li) in document
        .find(Attr("class", "result").and(Name("li")))
        .enumerate()
    {
        if let Some(href) = li.find(Name("a")).next().and_then(|a| a.attr("href")) {
            println!("\x1b[32mLink {} found on Ahmia: {}\x1b[0m", i + 1, href);
            found_match = true;
        }
    }

    if !found_match {
        println!("No match found for query: {}", query);
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <search_query>", args[0]);
        return;
    }
    let query = &args[1];
    if let Err(e) = search_dark_web(query).await {
        eprintln!("Error: {}", e);
    }
}
