/// The cli module handles command line argument parsing
pub mod cli;

/// The core module handles the engine that drives the log reader and retrieves the parsed JSON
pub mod core;

/// The logs module handles locating and reading from CS:GO server log files
mod logs;

/// The rgx module handles the regular expressions for the various kinds of server log entries
mod rgx;

/// The geo module handles map-related functions, such as distance calculations, and conversions between the in-game coordinate system and decimal degrees
mod geo;

/// The state module handles tracking of the game state for a given match
mod state;

/// The utils module provides various utility functions such as Steam ID translation
mod utils;
