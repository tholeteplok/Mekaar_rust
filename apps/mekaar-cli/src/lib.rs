//! # mekaar-cli: Library exports for diagnostics and E2E simulation

pub mod args;
pub mod interactive;
pub mod simulation;

pub use args::{Cli, Commands};
pub use simulation::run_simulation;
