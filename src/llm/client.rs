use reqwest::Client;
use std::time::Duration;

#[derive(Debug)]
pub struct LlmClient {
    client: Client,
    api_key: String,
}

impl LlmClient {
    pub fn new(api_key: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(120))
            .build()
            .expect("Failed to create HTTP client");
        
        Self { client, api_key }
    }
    
    pub fn client(&self) -> &Client {
        &self.client
    }
    
    pub fn api_key(&self) -> &str {
        &self.api_key
    }
}
