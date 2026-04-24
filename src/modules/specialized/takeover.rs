use crate::error::Result;
use reqwest::Client;

pub struct TakeoverChecker {
    pub client: Client,
}

impl TakeoverChecker {
    pub async fn check(&self, domain: &str, cname: &str) -> Result<Option<String>> {
        if cname.contains("github.io") {
            let resp = self.client.get(format!("https://{}", domain)).send().await?;
            let body = resp.text().await?;
            if body.contains("There isn't a GitHub Pages site here") {
                return Ok(Some("GitHub Pages".to_string()));
            }
        }
        
        if cname.contains("herokudns.com") {
            let resp = self.client.get(format!("https://{}", domain)).send().await?;
            if resp.status().as_u16() == 404 {
                return Ok(Some("Heroku".to_string()));
            }
        }

        Ok(None)
    }
}
