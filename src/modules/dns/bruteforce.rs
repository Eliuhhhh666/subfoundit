use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::Arc;
use tokio::sync::mpsc;
use async_trait::async_trait;

use crate::modules::Module;
use crate::model::Subdomain;
use crate::error::Result;
use crate::modules::dns::resolver::Resolver;

pub struct Bruteforcer {
    pub wordlist_path: String,
    pub resolver: Arc<Resolver>,
}

#[async_trait]
impl Module for Bruteforcer {
    fn name(&self) -> String { "Bruteforcer".to_string() }
    fn description(&self) -> String { "Active DNS bruteforcing".to_string() }

    async fn run(&self, target: &str) -> Result<Vec<Subdomain>> {
        let (tx, mut rx) = mpsc::channel(1000);
        let mut results = vec![];

        let path = self.wordlist_path.clone();
        tokio::spawn(async move {
            if let Ok(file) = File::open(path) {
                let reader = BufReader::new(file);
                for line in reader.lines() {
                    if let Ok(word) = line {
                        if tx.send(word).await.is_err() { break; }
                    }
                }
            }
        });

        while let Some(word) = rx.recv().await {
            let sub_name = format!("{}.{}", word, target);
            if self.resolver.resolve(&sub_name).await {
                results.push(Subdomain {
                    name: sub_name,
                    source: self.name(),
                });
            }
        }
        Ok(results)
    }
}
