use reqwest::Client;

pub struct CloudDiscovery {
    pub client: Client,
}

impl CloudDiscovery {
    pub async fn check_bucket(&self, domain: &str, cname: &str) {
        match cname {
            c if c.contains("s3.amazonaws.com") => {
                self.probe_s3(domain).await;
            },
            c if c.contains("storage.googleapis.com") => {
                self.probe_gcp(domain).await;
            },
            _ => { /* Do nothing */ }
        }
    }

    async fn probe_s3(&self, bucket_name: &str) {
        let url = format!("https://{}.s3.amazonaws.com", bucket_name);
        if let Ok(resp) = self.client.get(&url).send().await {
            println!("[CLOUD] S3 Bucket Found: {} (Status: {})", url, resp.status());
        }
    }

    async fn probe_gcp(&self, bucket_name: &str) {
        let url = format!("https://storage.googleapis.com/{}", bucket_name);
        if let Ok(resp) = self.client.get(&url).send().await {
            println!("[CLOUD] GCP Bucket Found: {} (Status: {})", url, resp.status());
        }
    }
}
