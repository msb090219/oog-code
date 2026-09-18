//! Application module for Oog Code
//!
//! This module will contain the main LiveCli application logic extracted from main.rs.
//! Currently serves as a placeholder for the upcoming refactoring (Phase 0.1).
//!
//! # TODO (Phase 0.1)
//!
//! - Extract `LiveCli` struct from main.rs (currently at line 1799, ~7,447 lines total)
//! - Extract `format_*` helper functions to `format.rs`
//! - Extract session management to `session_manager.rs`
//! - Move REPL loop logic here
//!
//! # Legacy Code Removed
//!
//! The previous `CliApp` struct was an earlier prototype using `ConversationClient`
//! that has been superseded by the more sophisticated `LiveCli` in main.rs.
//! The legacy code has been backed up to `app.rs.legacy_backup` for reference if needed.

// Re-export common types that will be used after refactoring
pub use crate::args::{OutputFormat, PermissionMode};

// This module is intentionally minimal until Phase 0.1 refactoring
