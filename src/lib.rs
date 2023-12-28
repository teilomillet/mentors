// src/lib.rs
pub mod components;
pub mod app; // Ensure the app module is public so it can be accessed from outside
pub use app::App; // Re-export App for ease of use in main.rs
