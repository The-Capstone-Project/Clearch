// NOTE:
// GPT-4 Model 
use reqwest::Client;
use crate::traits::LLMRequest;



struct GPT4Model {
    client: Client,
    api_key: String,
}

impl LLMRequest for GPT4Model {
    fn new(api_key: String) -> Self {
        GPT4Model{ client: Client::new(), api_key }
    }
    async fn req(&self) -> Result<String, reqwest::Error> {
        let url = "https://api.openai.com/v4/completions";
        let response = self.client.post(url)
            .bearer_auth(&self.api_key)
            .json(&serde_json::json!({
                "model": "text-davinci-003", // Replace with the appropriate GPT-4 model identifier
                "prompt": "Your prompt here",
                "max_tokens": 100,
            }))
            .send()
            .await?
            .text()
            .await?;
        Ok(response)
    }
}

