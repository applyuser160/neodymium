use crate::engines::RenderingEngine;
use reqwest::blocking::Client;
use std::time::Duration;

pub struct HttpEngine {
    #[allow(dead_code)]
    client: Client,
}

impl Default for HttpEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpEngine {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .user_agent("Neodymium/0.1")
                .timeout(Duration::from_secs(10))
                .build()
                .unwrap_or_default(),
        }
    }
}

impl RenderingEngine for HttpEngine {
    fn name(&self) -> &str {
        "HTTP Engine"
    }

    fn render(&self, url: &str) -> String {
        match self.client.get(url).send() {
            Ok(response) => match response.text() {
                Ok(text) => text,
                Err(e) => format!("Error reading body: {}", e),
            },
            Err(e) => format!("Error connecting: {}", e),
        }
    }
}
