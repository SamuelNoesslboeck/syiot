#[warn(missing_docs)]

extern crate alloc;

// Submodules
/// Structs and types for telemtry
pub mod tele;

// Custom types
pub type Error = Box<dyn std::error::Error>;