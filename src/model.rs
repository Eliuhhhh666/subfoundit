use serde::{Deserialize, Serialize};

// #[derive] gives our struct "Quick-Skills" like being able to
// be cloned or turned into JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subdomain {
    pub name: String, // The domain Name (e.g, "Example.com")
    pub source: String, // The module that found it (e.g, "Crtsh")
}