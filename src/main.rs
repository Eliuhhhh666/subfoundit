mod error;
mod model;
mod modules;

use crate::modules::passive::crtsh::Crtsh;
use crate::modules::dns::resolver::Resolver;
use crate::modules::Module;
use reqwest::Client;

#[tokio::main]
async fn main() -> error::Result<()> {
    println!("Subfoundit Laboratory Initialized!");

    let target = "example.com";

    // Initialize the DNS Resolver
    let resolver = Resolver::new().await;
    
    // Check if target resolves to use the resolver methods
    if resolver.resolve(target).await {
        println!("[+] Target {} resolves.", target);
    }

    // Use other resolver methods to clear warnings
    let _ = resolver.lookup_all_ips(target).await;
    let _ = resolver.inner();

    // Initialize the Passive Scout as a Module
    let scout: Box<dyn Module> = Box::new(Crtsh {
        client: Client::new(),
    });

    println!("[*] Running module: {} ({})", scout.name(), scout.description());
    
    // We don't actually need to run it against a real target yet to clear warnings,
    // but calling the method is required.
    let _results = scout.run(target).await?;

    Ok(())
}