use std::error::Error;
use reqwest::Client;
use crate::traits::LLMRequest;

struct ClaudeModel {
    client: Client,
    api_key: String,
}

impl LLMRequest for ClaudeModel {
    fn new(api_key: String) -> Self {
        ClaudeModel{ client: Client::new(), api_key }
    }
    async fn req(&self, query: &str, fine: &str) -> Result<String, Box<dyn Error>> {
        let url = "https://api.anthropic.com/v1/completions"; // The Claude API endpoint
        let response = self.client.post(url)
            .bearer_auth(&self.api_key)
            .json(&serde_json::json!({
                "model": fine, // Use the `fine` parameter for the model identifier
                "prompt": query, // Use the `query` parameter for the prompt
                "max_tokens": 100,
            }))
            .send()
            .await?;
    
        let text = response.text().await?;
        Ok(text)
    }
    
}
