use futures_util::StreamExt;
use tokio_tungstenite::connect_async;
use serde_json::Value;

pub struct CertStreamer;

impl CertStreamer {
    pub async fn stream(&self, target_keyword: &str) {
        let url = "wss://certstream.calidog.io/";
        let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
        let (_, mut read) = ws_stream.split();

        while let Some(msg) = read.next().await {
            if let Ok(message) = msg {
                if let Ok(data) = serde_json::from_str::<Value>(&message.to_string()) {
                    if let Some(domains) = data.get("data")
                        .and_then(|d| d.get("leaf_cert"))
                        .and_then(|l| l.get("all_domains"))
                        .and_then(|a| a.as_array()) 
                    {
                        for domain in domains {
                            let domain_str = domain.as_str().unwrap_or("");
                            if domain_str.contains(target_keyword) {
                                println!("[CT STREAM] Found: {}", domain_str);
                            }
                        }
                    }
                }
            }
        }
    }
}
