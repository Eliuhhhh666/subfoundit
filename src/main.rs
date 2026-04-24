mod error;
mod model;
mod modules;

use crate::modules::passive::crtsh::Crtsh;
use crate::modules::dns::resolver::Resolver;
use crate::modules::dns::wildcard::WildcardFilter;
use crate::modules::dns::bruteforce::Bruteforcer;
use crate::modules::dns::permutation::Permutator;
use crate::modules::Module;
use reqwest::Client;
use std::sync::Arc;

#[tokio::main]
async fn main() -> error::Result<()> {
    println!("Subfoundit Laboratory Initialized!");

    let target = "example.com";

    // Initialize the DNS Resolver
    let resolver = Arc::new(Resolver::new().await);
    
    // Check if target resolves to use the resolver methods
    if resolver.resolve(target).await {
        println!("[+] Target {} resolves.", target);
    }

    // Initialize Wildcard Filter
    let filter = WildcardFilter::detect(target, &resolver).await;
    let _ = filter.is_wildcard(&[]);
    let _ = resolver.inner();

    // Initialize Permutator
    let permutator = Permutator { keywords: vec!["dev".to_string()] };
    let _mutations = permutator.generate("api.example.com");

    // Initialize the Passive Scout as a Module
    let scout: Box<dyn Module> = Box::new(Crtsh {
        client: Client::new(),
    });

    println!("[*] Running module: {} ({})", scout.name(), scout.description());
    let _results = scout.run(target).await?;

    // Initialize the Bruteforcer
    let bf: Box<dyn Module> = Box::new(Bruteforcer {
        wordlist_path: "words.txt".to_string(),
        resolver: Arc::clone(&resolver),
    });

    println!("[*] Running module: {} ({})", bf.name(), bf.description());
    // We won't run it here to avoid needing a real words.txt file for check
    // let _bf_results = bf.run(target).await?;

    Ok(())
}