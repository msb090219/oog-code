//! Formatting and reporting functions
//!
//! This module contains all the `format_*` helper functions that generate
//! human-readable reports for the CLI. Extracted from main.rs during Phase 0.1.
//!
//! # TODO
//!
//! This is a placeholder - the actual format_* functions will be moved here
//! during the refactoring.

use std::path::Path;

/// Format an error message for unknown command-line option
pub fn format_unknown_option(option: &str) -> String {
    if option.starts_with('-') {
        format!("unknown option {option}")
    } else {
        format!("unknown command or argument: {option}")
    }
}

// More format_* functions will be added here during refactoring
