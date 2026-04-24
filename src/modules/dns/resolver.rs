use trust_dns_resolver::TokioAsyncResolver;
use trust_dns_resolver::config::*;
use std::net::IpAddr;

pub struct Resolver {
    resolver: TokioAsyncResolver,
}

impl Resolver {
    pub async fn new() -> Self {
        let resolver = TokioAsyncResolver::tokio(
            ResolverConfig::cloudflare(),
            ResolverOpts::default(),
        );
        
        Self { resolver }
    }

    pub async fn resolve(&self, domain: &str) -> bool {
        match self.resolver.lookup_ip(domain).await {
            Ok(ips) => {
                ips.iter().next().is_some()
            },
            Err(_) => false,
        }
    }

    // New: Support for Wildcard detection
    pub async fn lookup_all_ips(&self, domain: &str) -> crate::error::Result<Vec<IpAddr>> {
        let lookup = self.resolver.lookup_ip(domain).await?;
        Ok(lookup.iter().collect())
    }

    // New: Access the inner resolver for specialized tasks (like PTR sweeps)
    pub fn inner(&self) -> TokioAsyncResolver {
        self.resolver.clone()
    }
}
