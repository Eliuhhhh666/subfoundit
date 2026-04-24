use std::collections::HashSet;
use std::net::IpAddr;
use crate::modules::dns::resolver::Resolver;

pub struct WildcardFilter {
    ignored_ips: HashSet<IpAddr>,
}

impl WildcardFilter {
    pub async fn detect(target: &str, resolver: &Resolver) -> Self {
        let mut ignored_ips = HashSet::new();
        let random_sub = format!("z9x2j4-71l0p2.{}", target);
        
        if let Ok(ips) = resolver.lookup_all_ips(&random_sub).await {
            for ip in ips {
                ignored_ips.insert(ip);
            }
        }
        
        Self { ignored_ips }
    }

    pub fn is_wildcard(&self, found_ips: &[IpAddr]) -> bool {
        if self.ignored_ips.is_empty() { return false; }
        found_ips.iter().any(|ip| self.ignored_ips.contains(ip))
    }
}
