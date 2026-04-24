// LINE 1: We borrow the 'async_trait' plugin.
use async_trait::async_trait;

// LINE 2-4: We reach into our own project folders.
use crate::modules::Module;
use crate::model::Subdomain;
use crate::error::Result;

// LINE 5-6: Our HTTP tools and JSON translator.
use reqwest::Client;
use serde::Deserialize;

// LINE 7-10: Our JSON Data Model.
#[derive(Deserialize)]
struct CrtshResult {
    name_value: String,
}

// LINE 11-13: Our Module structure.
pub struct Crtsh {
    pub client: Client,
}

// LINE 14-36: Signing the Contract.
#[async_trait]
impl Module for Crtsh {
    fn name(&self) -> String { "Crtsh".to_string() }
    fn description(&self) -> String { "Queries crt.sh for CT logs".to_string() }

    async fn run(&self, target: &str) -> Result<Vec<Subdomain>> {
        let url = format!("https://crt.sh/?q=%.{}&output=json", target);
        
        let resp = self.client.get(url)
            .send().await?
            .json::<Vec<CrtshResult>>().await?;
        
        let subdomains = resp.into_iter()
            .map(|r| Subdomain { 
                name: r.name_value, 
                source: self.name() 
            })
            .collect();

        Ok(subdomains)
    }
}
