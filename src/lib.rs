#[deny(missing_docs)]

// Submodules
mod tele;

// Custom types
pub type Error = Box<dyn std::error::Error>;