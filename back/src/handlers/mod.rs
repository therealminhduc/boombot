pub mod health;
pub mod rules;
pub mod admin;

// Re-export all handlers for easy importing
pub use health::*;
pub use rules::*;
pub use admin::*;
