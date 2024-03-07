use clap::Parser;
use dotenv::dotenv;
use reqwest::header;
use serde_json::Value;
use std::{collections::HashMap, vec};

#[derive(Parser, Debug)]
#[command(name = "Clearch")]
#[command(author = "Advaith Narayanan <advaith@glitchy.systems>")]
#[command(version = "1.0")]
#[command(about = "Search using the command line", long_about = None)]
struct Gemini {
    query: Option<String>
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let search = Gemini::parse();

    print!("{:?}", search.query.as_deref());

    if let Some(query) = search.query.as_deref() {
        let mut map = HashMap::new();
        map.insert(
            "contents",
            vec![HashMap::from([(
                "parts",
                vec![HashMap::from([("text", format!("{}", query))])],
            )])],
        );

        // Gemini part

        let apikey = env!("GEMIAI_API");
        let client = reqwest::Client::new();
        let resp = client.post(format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent?key={apikey}"))
    .header(header::CONTENT_TYPE, "application/json")
        .json(&map)
        .send()
        .await?;

        let json: Value = resp.json().await?;

        if let Some(candidate) = json["candidates"].get(0) {
            if let Some(content) = candidate["content"]["parts"].get(0) {
                if let Some(text) = content["text"].as_str() {
                    println!("Answer: \n{}", text);
                }
            }
        }
    }
    Ok(())
}
