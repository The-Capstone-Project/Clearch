use clap::Parser;
use reqwest::header;
use serde_json::Value;
use std::{collections::HashMap, vec};
use dotenv::dotenv;

#[derive(Parser, Debug)]
#[command(name = "Clearch")]
#[command(author = "Advaith Narayanan <advaith@glitchy.systems>")]
#[command(about = "Search using the command line")]
struct Gemini {
    query: Option<String>,

    #[clap(long, short)]
    query_adn: Option<bool>
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let client = reqwest::Client::new();
    let search = Gemini::parse();
    
    let apikey = match std::env::var("GEMIAI_API") {
        Ok(apikey) => apikey,
        Err(e) => panic!("API not found: {}", e)
    };
    
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
        // if let apikey = env!("GEMIAI_API"){}

        let resp = client.post(format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent?key={}",apikey))
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
    }else if search.query_adn.unwrap_or(false){

    }
    Ok(())
}


fn gemini_req(){

}