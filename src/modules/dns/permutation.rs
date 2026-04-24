pub struct Permutator {
    pub keywords: Vec<String>,
}

impl Permutator {
    pub fn generate(&self, discovered: &str) -> Vec<String> {
        let mut mutations = vec![];
        let parts: Vec<&str> = discovered.split('.').collect();
        if parts.len() < 2 { return mutations; }
        
        let sub = parts[0]; 
        let root = parts[1..].join(".");

        for word in &self.keywords {
            mutations.push(format!("{}-{}.{}", sub, word, root));
            mutations.push(format!("{}.{}.{}", word, sub, root));
        }

        mutations
    }
}
