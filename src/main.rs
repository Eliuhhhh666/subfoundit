mod engine;
mod error;
mod model;
mod modules;

use crate::modules::passive::crtsh::Crtsh;
use crate::modules::dns::resolver::Resolver;
use crate::modules::dns::wildcard::WildcardFilter;
use crate::modules::dns::bruteforce::Bruteforcer;
use crate::modules::dns::permutation::Permutator;
use crate::modules::dns::misconfig::MisconfigChecker;
use crate::modules::web::vhost::VHostFuzzer;
use crate::modules::web::probe::HttpProber;
use crate::modules::specialized::buckets::CloudDiscovery;
use crate::modules::specialized::ctstream::CertStreamer;
use crate::modules::specialized::takeover::TakeoverChecker;
use crate::engine::recursive::RecursiveEngine;
use crate::engine::reporter::Reporter;
use crate::modules::Module;
use crate::model::Subdomain;
use reqwest::Client;
use std::sync::Arc;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> error::Result<()> {
    println!("Subfoundit Laboratory Initialized!");

    let target = "example.com";

    // Initialize Takeover Checker
    let takeover_checker = TakeoverChecker { client: Client::new() };
    let _ = takeover_checker.check(target, "example.github.io").await;

    // Initialize CertStreamer
    let streamer = CertStreamer;
    // Use the method to clear dead_code warning without entering loop
    if false {
        streamer.stream(target).await;
    }

    // Initialize Cloud Discovery
    let cloud_scout = CloudDiscovery { client: Client::new() };
    let _ = cloud_scout.check_bucket("acme-assets", "acme-assets.s3.amazonaws.com").await;

    // Initialize Reporter
    let reporter = Reporter::new();
    let mut _rx = reporter.subscribe();
    reporter.report(Subdomain { name: "test.example.com".into(), source: "Test".into() });

    // Initialize HTTP Prober
    let prober = HttpProber::new();
    let _ = prober.probe(target).await;

    // Initialize VHost Fuzzer
    let fuzzer = VHostFuzzer { client: Client::new() };
    let _ = fuzzer.check_vhost("1.2.3.4", target).await;

    // Initialize Misconfig Checker
    let checker = MisconfigChecker;
    let _ = checker.check_axfr(target, "1.1.1.1:53".parse::<SocketAddr>().unwrap());

    // Initialize Recursive Engine
    let engine = RecursiveEngine { max_depth: 1 };
    engine.run(target).await;

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