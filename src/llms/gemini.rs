use reqwest::{header, Client};
use serde_json::{json, Value};
use std::error::Error;
use crate::traits::LLMRequest;


pub struct GeminiModel {
    api_key: String,
    client: Client,
}

impl LLMRequest for GeminiModel {
    fn new(api_key: String) -> Self {
        GeminiModel { client: Client::new(), api_key }
    }

    async fn req(&self, query: &str, fine: &str) -> Result<String, Box<dyn Error>> {
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

       
        let resp = self.client.post(format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent?key={}", self.api_key))
            .header(header::CONTENT_TYPE, "application/json")
            .json(&map)
            .send()
            .await?;

        let json: Value = resp.json().await?;

        if let Some(candidate) = json["candidates"].get(0) {
            if let Some(content) = candidate["content"]["parts"].get(0) {
                if let Some(text) = content["text"].as_str() {
                    println!("{}", text);
                    return Ok(text.to_string()); 
                }
            }
        }
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Invalid")))
    }
}




