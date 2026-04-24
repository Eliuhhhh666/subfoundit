use tokio::sync::broadcast;
use crate::model::Subdomain;

pub struct Reporter {
    tx: broadcast::Sender<Subdomain>,
}

impl Reporter {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(100);
        Self { tx }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Subdomain> {
        self.tx.subscribe()
    }

    pub fn report(&self, sub: Subdomain) {
        let _ = self.tx.send(sub);
    }
}
