pub mod dns;
pub mod passive;
pub mod specialized;
pub mod web;

use async_trait::async_trait;
use crate::model::Subdomain;
use crate::error::Result;

#[async_trait]
pub trait Module: Send + Sync {
    fn name(&self) -> String;
    fn description(&self) -> String;
    async fn run (&self, target: &str) -> Result<Vec<Subdomain>>;
}