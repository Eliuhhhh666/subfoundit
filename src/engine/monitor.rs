use std::collections::HashSet;

pub struct Monitor;

impl Monitor {
    pub fn find_new_assets(&self, old_scan: Vec<String>, current_scan: Vec<String>) -> Vec<String> {
        let old_set: HashSet<String> = old_scan.into_iter().collect();
        let current_set: HashSet<String> = current_scan.into_iter().collect();

        current_set
            .difference(&old_set)
            .cloned()
            .collect()
    }

    pub async fn alert(&self, webhook_url: &str, new_sub: &str) {
        let client = reqwest::Client::new();
        let payload = serde_json::json!({
            "text": format!("🚨 NEW ASSET DISCOVERED: `{}`", new_sub)
        });
        let _ = client.post(webhook_url).json(&payload).send().await;
    }
}
