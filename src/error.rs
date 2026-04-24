use anyhow::Error;

// This nickname (alias) lets us write 'Result<T>' instead of 
// a long, confusing type name in every file.
pub type Result<T> = std::result::Result<T, Error>;