use trust_dns_resolver::TokioAsyncResolver;

pub struct AsnMapper {
    pub resolver: TokioAsyncResolver,
}

impl AsnMapper {
    pub async fn sweep(&self, cidr: &str) -> Vec<String> {
        let mut subdomains = vec![];
        if let Ok(network) = cidr.parse::<ipnetwork::IpNetwork>() {
            for ip in network.iter() {
                if let Ok(lookup) = self.resolver.reverse_lookup(ip).await {
                    for name in lookup.iter() {
                        let domain = name.to_string();
                        println!("[PTR] Found {} at IP {}", domain, ip);
                        subdomains.push(domain);
                    }
                }
            }
        }
        subdomains
    }
}
