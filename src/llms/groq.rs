use crate::traits::LLMRequest;
use reqwest::Client;
use serde_json::json;
use std::error::Error;

pub struct GroqModel {
    client: Client,
    api_key: String,
}

impl LLMRequest for GroqModel {
    fn new(api_key: String) -> Self {
        GroqModel {
            client: Client::new(),
            api_key,
        }
    }

    async fn req(&self, query: &str, fine: &str) -> Result<String, Box<dyn Error>> {
        let url = "https://api.groq.com/openai/v1/chat/completions";
        
        let response = self.client.post(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&json!({
                "model": "llama3-70b-8192", // Default model, can be parameterized if needed
                "messages": [
                    {
                        "role": "system",
                        "content": fine
                    },
                    {
                        "role": "user",
                        "content": query
                    }
                ],
                "temperature": 0.7,
                "max_tokens": 4096
            }))
            .send()
            .await?;

        let response_text = response.text().await?;
        Ok(response_text)
    }
}