//! Command-line argument types and enums
//!
//! This module contains the type definitions used for command-line argument parsing.
//! The actual parsing logic is in main.rs (hand-rolled parser) which is more
//! feature-complete than the clap-based parser that was previously here.
//!
//! # TODO (Phase 0.1)
//!
//! As part of the big refactoring, the hand-rolled parser from main.rs should be
//! moved here, making this a complete argument parsing module.

use std::path::PathBuf;

/// Permission mode for tool execution
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionMode {
    /// No filesystem writes allowed
    ReadOnly,
    /// Writes only within workspace directory
    WorkspaceWrite,
    /// Unrestricted access
    DangerFullAccess,
}

/// Output format for responses
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CliOutputFormat {
    /// Plain text output
    Text,
    /// JSON output
    Json,
}

impl CliOutputFormat {
    /// Parse output format from string
    pub fn parse(value: &str) -> Result<Self, String> {
        match value {
            "text" => Ok(Self::Text),
            "json" => Ok(Self::Json),
            other => Err(format!(
                "unsupported value for --output-format: {other} (expected text or json)"
            )),
        }
    }
}

/// CLI action to perform (result of argument parsing)
///
/// This represents the high-level action that the CLI should take based on
/// the command-line arguments provided.
///
/// Note: This is simplified version - the full version with all variants
/// is defined in main.rs. Will be consolidated during Phase 0.1 refactoring.
#[derive(Debug, Clone)]
pub enum CliAction {
    /// Show help message
    Help,

    /// Show version information
    Version,

    /// Start interactive REPL
    Repl {
        model: String,
        allowed_tools: Option<AllowedToolSet>,
        permission_mode: PermissionMode,
    },

    /// Run a one-shot prompt
    Prompt {
        prompt: String,
        model: String,
        output_format: CliOutputFormat,
        allowed_tools: Option<AllowedToolSet>,
        permission_mode: PermissionMode,
    },

    /// Show status information
    Status {
        model: String,
        permission_mode: PermissionMode,
    },

    /// Login with OAuth
    Login,

    /// Logout and clear credentials
    Logout,

    /// Initialize repository
    Init,

    /// Show sandbox status
    Sandbox,

    /// Dump upstream manifests
    DumpManifests,

    /// Show bootstrap plan
    BootstrapPlan,

    /// Run agents command
    Agents {
        args: Option<String>,
    },

    /// Run MCP command
    Mcp {
        args: Option<String>,
    },

    /// Run skills command
    Skills {
        args: Option<String>,
    },
}

/// Verbosity level for tool output
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verbosity {
    /// Show only tool names
    Terse,
    /// Show tool names with brief summaries (default)
    Normal,
    /// Show full tool output
    Verbose,
}

impl Default for Verbosity {
    fn default() -> Self {
        Self::Normal
    }
}

impl Verbosity {
    pub fn parse(value: &str) -> Result<Self, String> {
        match value {
            "terse" => Ok(Self::Terse),
            "normal" => Ok(Self::Normal),
            "verbose" => Ok(Self::Verbose),
            other => Err(format!(
                "unsupported verbosity level: {other} (expected terse, normal, or verbose)"
            )),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Terse => "terse",
            Self::Normal => "normal",
            Self::Verbose => "verbose",
        }
    }
}

/// Set of allowed tools (whitelist mode)
#[derive(Debug, Clone)]
pub struct AllowedToolSet {
    pub tools: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_format_parse() {
        assert_eq!(CliOutputFormat::parse("text"), Ok(CliOutputFormat::Text));
        assert_eq!(CliOutputFormat::parse("json"), Ok(CliOutputFormat::Json));
        assert!(CliOutputFormat::parse("invalid").is_err());
    }

    #[test]
    fn test_permission_mode_equality() {
        assert_eq!(PermissionMode::ReadOnly, PermissionMode::ReadOnly);
        assert_ne!(PermissionMode::ReadOnly, PermissionMode::WorkspaceWrite);
    }

    #[test]
    fn test_verbosity_default() {
        assert_eq!(Verbosity::default(), Verbosity::Normal);
    }

    #[test]
    fn test_verbosity_parse() {
        assert_eq!(Verbosity::parse("terse"), Ok(Verbosity::Terse));
        assert_eq!(Verbosity::parse("normal"), Ok(Verbosity::Normal));
        assert_eq!(Verbosity::parse("verbose"), Ok(Verbosity::Verbose));
        assert!(Verbosity::parse("invalid").is_err());
    }

    #[test]
    fn test_verbosity_name() {
        assert_eq!(Verbosity::Terse.name(), "terse");
        assert_eq!(Verbosity::Normal.name(), "normal");
        assert_eq!(Verbosity::Verbose.name(), "verbose");
    }
}
