use clap::Parser;
use clap_stdin::FileOrStdin;
use dotenv::dotenv;
use reqwest::{header, Client};
use serde_json::{json, Value};
// use std::io::{self, Read};
use sys_info::{os_release, os_type};

#[derive(Parser, Debug)]
#[command(name = "Clearch")]
#[command(author = "Advaith Narayanan <advaith@glitchy.systems>")]
#[command(about = "Search using the command line")]
struct Gemini {
    #[clap(required_unless_present_all = "query-script")]
    query: Option<String>,
    
    query_script: FileOrStdin<String>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    println!(
        "OS: {}  OS REL: {} ",
        os_type().unwrap(),
        os_release().unwrap()
    );

    let search = Gemini::parse();

    let apikey = match std::env::var("GEMIAI_API") {
        Ok(apikey) => apikey,
        Err(e) => panic!("API not found: {}", e),
    };

    // #[cfg(not(debug_assertions))]
    // if let apikey = env!("GEMIAI_API"){}

    if let Some(query) = search.query.as_deref() {
        println!("Searching for: {}", query);
        req(query, apikey, "").await.unwrap();
    } else {
        // match search.query_script {
        //     Some(z) => {
        //         println!("Searching for: {}", z);
        //         req(&z, apikey,format!("OS: {}  OS REL: {} ", os_type().unwrap(), os_release().unwrap()).as_str()).await.unwrap();
        //     }
        //     None => eprintln!("Failed to read from stdin"),
        // }

        // match search.query_script.contents() {
        //     Ok(z) => {
        //         println!("Searching for: {}", z);
        //         req(&z, apikey,format!("OS: {}  OS REL: {} ", os_type().unwrap(), os_release().unwrap()).as_str()).await.unwrap();
        //     }
        //     Err(_) => eprintln!("Failed to read from stdin"),
        // }

        if let Ok(buffer) = search.query_script.contents() {
            // let mut buffer = String::new();
            // println!("{buffer}");
            // io::stdin()
            //     .read_to_string(&mut buffer)
            //     .expect("Failed to read from stdin");
            // println!("Searching for: {}", buffer);
            req(
                &buffer,
                apikey,
                format!(
                    "OS: {}  OS REL: {} Using this information return the syntax for proper bash script without any markdown or html just raw bash script",
                    os_type().unwrap(),
                    os_release().unwrap()
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
