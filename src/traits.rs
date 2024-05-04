// src/traits.rs

use std::error::Error;

pub trait LLMRequest {
    async fn req(&self, query: &str, fine: &str) -> Result<String, Box<dyn Error>>;
    fn new(api_key: String) -> Self;
}

