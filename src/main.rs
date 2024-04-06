use clap::Parser;
use clap_stdin::FileOrStdin;
use dotenv::dotenv;
use reqwest::{header, Client};
use serde_json::{json, Value};
// use std::io::{self, Read};
use sys_info::{linux_os_release, os_release, os_type};

#[derive(Parser)]
#[command(name = "Clearch")]
#[command(author = "Advaith Narayanan <advaith@glitchy.systems>")]
#[command(about = "Search using the command line")]
struct Gemini {
    
    #[arg(short='q',long="specify")]
    search_query: Option<String>,
    
    // #[arg(long="query",help = "Pass `-h` and you'll see me!")]
    query: FileOrStdin<String>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    println!(
        "OS: {}  OS REL: {} Linux: {} ",
        os_type().unwrap(),
        os_release().unwrap(),
        linux_os_release().unwrap().pretty_name()
    );

    let search = Gemini::parse();

    let apikey = match std::env::var("GEMIAI_API") {
        Ok(apikey) => apikey,
        Err(e) => panic!("API not found: {}", e),
    };

    // #[cfg(not(debug_assertions))]
    // if let apikey = env!("GEMIAI_API"){}

    if let Some(query) = search.search_query.as_deref() {
        println!("Searching for: {}", query);
        req(query, apikey, "").await.unwrap();
    } else {
        if let Ok(buffer) = search.query.contents() {
            println!("{buffer}");
            let prompt: Vec<String> = include_str!("prompt").split("\n").map(|s| s.to_string()).collect();
            println!("lenth of array {}",prompt.len());
            for i in prompt{
                println!("{}", i);
            }
            req(
                &buffer,
                apikey,
                format!(
                    "OS: {}  kernal version: {}, 
                    {}",
                    os_type().unwrap(),
                    os_release().unwrap(),
                    linux_os_release().unwrap().pretty_name()
                )
                .as_str(),
            )
            .await
            .unwrap();
        }
    }
}

async fn req(query: &str, apikey: String, fine: &str) -> Result<(), Box<dyn std::error::Error>> {
    let map = json!({
    "contents": [
        {
            "parts": [
                {
                    "text": format!("{} {}", query, fine)
                }
                ]
        }
        ]
    });

    let client = Client::new();
    let resp = client.post(format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent?key={}",apikey))
    .header(header::CONTENT_TYPE, "application/json")
    .json(&map)
    .send()
    .await?;

    let json: Value = resp.json().await?;

    if let Some(candidate) = json["candidates"].get(0) {
        if let Some(content) = candidate["content"]["parts"].get(0) {
            if let Some(text) = content["text"].as_str() {
                println!("{}", text);
            }
        }
    }
    Ok(())
}
