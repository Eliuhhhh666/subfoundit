use std::time::Duration;
use reqwest::Client;

pub struct HttpProber {
    client: Client,
}

impl HttpProber {
    pub fn new() -> Self {
        let client = Client::builder()
            .danger_accept_invalid_certs(true)
            .timeout(Duration::from_secs(5))
            .build()
            .expect("Failed to build HTTP Client");
        
        Self { client }
    }

    pub async fn probe(&self, domain: &str) -> Vec<String> {
        let mut live_urls = vec![];
        let protocols = ["https", "http"];

        for proto in protocols {
            let url = format!("{}://{}", proto, domain);
            if let Ok(resp) = self.client.get(&url).send().await {
                println!("[PROBE] Found {} (Status: {})", url, resp.status());
                live_urls.push(url);
            }
        }
        live_urls
    }
}
