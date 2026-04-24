use reqwest::header::{HeaderMap, HeaderValue, HOST};
use reqwest::Client;

pub struct VHostFuzzer {
    pub client: Client,
}

impl VHostFuzzer {
    pub async fn check_vhost(&self, ip: &str, candidate: &str) -> bool {
        let baseline = self.client.get(format!("http://{}", ip)).send().await;
        if baseline.is_err() { return false; }
        let baseline_resp = baseline.unwrap();
        let baseline_len = baseline_resp.content_length().unwrap_or(0);

        let mut headers = HeaderMap::new();
        if let Ok(val) = HeaderValue::from_str(candidate) {
            headers.insert(HOST, val);
        }

        let resp = self.client.get(format!("http://{}", ip))
            .headers(headers)
            .send().await;

        if let Ok(r) = resp {
            let current_len = r.content_length().unwrap_or(0);
            return current_len != baseline_len;
        }

        false
    }
}
