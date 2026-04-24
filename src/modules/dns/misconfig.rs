use trust_dns_client::client::{AsyncClient, ClientHandle};
use trust_dns_client::tcp::TcpClientStream;
use trust_dns_client::rr::{DNSClass, Name, RecordType};
use trust_dns_client::proto::xfer::DnsMultiplexer;
use trust_dns_client::proto::iocompat::AsyncIoTokioAsStd;
use trust_dns_client::proto::op::NoopMessageFinalizer;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use tokio::net::TcpStream;

pub struct MisconfigChecker;

impl MisconfigChecker {
    pub async fn check_axfr(&self, domain: &str, nameserver: SocketAddr) -> crate::error::Result<Vec<String>> {
        let name = Name::from_str(domain)?;
        
        let (stream, handle) = TcpClientStream::<AsyncIoTokioAsStd<TcpStream>>::new(nameserver);
        let (mut client, bg) = AsyncClient::connect(DnsMultiplexer::new(stream, handle, None::<Arc<NoopMessageFinalizer>>)).await?;
        tokio::spawn(bg);

        let response = client.query(name, DNSClass::IN, RecordType::AXFR).await?;
        let mut subdomains = vec![];
        for record in response.answers() {
            subdomains.push(record.name().to_string());
        }
        Ok(subdomains)
    }
}
