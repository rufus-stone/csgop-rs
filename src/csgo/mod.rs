/// The cli module handles command line argument parsing
pub mod cli;

/// The core module handles the engine that drives the log reader, the game state and JSON generation
pub mod core;

/// The logs module handles locating and reading from CS:GO server log files
mod logs;
