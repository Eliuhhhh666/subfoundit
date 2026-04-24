use trust_dns_client::client::AsyncClient;
use trust_dns_client::tcp::TcpClientStream;
use trust_dns_client::rr::{DNSClass, Name, RecordType};
use futures_util::StreamExt;
use std::net::SocketAddr;
use std::str::FromStr;

pub struct MisconfigChecker;

impl MisconfigChecker {
    pub async fn check_axfr(&self, domain: &str, nameserver: SocketAddr) -> crate::error::Result<Vec<String>> {
        let (stream, handle) = TcpClientStream::new(nameserver);
        let (mut client, bg) = AsyncClient::connect(stream).await?;
        
        // Drive the background task
        tokio::spawn(bg);

        let name = Name::from_str(domain)?;
        let mut response = client.query(name, DNSClass::IN, RecordType::AXFR).await?;
        let mut subdomains = vec![];
        
        while let Some(record) = response.next().await {
            let r = record?;
            subdomains.push(r.name().to_string());
        }

        Ok(subdomains)
    }
}
