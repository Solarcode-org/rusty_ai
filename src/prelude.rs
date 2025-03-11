//! Prelude module.

/// Best error handling in the world.
pub type Result<T=(), E=color_eyre::Report> = color_eyre::Result<T, E>;

// color_eyre
pub use color_eyre::eyre::bail;
