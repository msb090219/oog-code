// Completion Engine for Auto-Suggestions
// Provides intelligent, context-aware suggestions for slash commands,
// argument values, and command history.

use commands::SlashCommandSpec;

/// Kind of suggestion being offered
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SuggestionKind {
    Command,
    Argument,
    SessionId,
    ModelName,
    PermissionMode,
    ThemeName,
    HistoryEntry,
}

/// A single suggestion with display text and replacement
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Suggestion {
    /// Text to display in the dropdown
    pub display: String,
    /// Text to insert when selected
    pub replacement: String,
    /// Optional description
    pub description: Option<String>,
    /// Kind of suggestion
    pub kind: SuggestionKind,
}

/// Context for completion
#[derive(Debug, Clone)]
pub struct CompletionContext {
    pub input: String,
    pub cursor_pos: usize,
}

impl CompletionContext {
    pub fn new(input: String, cursor_pos: usize) -> Self {
        Self { input, cursor_pos }
    }

    /// Parse the command context (what command are we in?)
    pub fn parse_command(&self) -> Option<&str> {
        let trimmed = self.input.trim();
        if trimmed.starts_with('/') {
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.is_empty() {
                None
            } else {
                Some(parts[0])
            }
        } else {
            None
        }
    }

    /// Get the prefix before cursor for completion
    pub fn get_prefix(&self) -> &str {
        &self.input[..self.cursor_pos]
    }
}

/// Completion engine for generating suggestions
pub struct CompletionEngine {
    specs: Vec<SlashCommandSpec>,
    sessions: Vec<String>,
    models: Vec<String>,
    themes: Vec<String>,
    history: Vec<String>,
}

impl CompletionEngine {
    pub fn new() -> Self {
        Self {
            specs: commands::slash_command_specs().to_vec(),
            sessions: Vec::new(),
            models: vec![
                "opus".to_string(),
                "sonnet".to_string(),
                "haiku".to_string(),
            ],
            themes: vec![
                "default".to_string(),
                "high-contrast".to_string(),
                "minimal".to_string(),
            ],
            history: Vec::new(),
        }
    }

    /// Update available sessions
    pub fn update_sessions(&mut self, sessions: Vec<String>) {
        self.sessions = sessions;
    }

    /// Update command history
    pub fn update_history(&mut self, history: Vec<String>) {
        self.history = history;
    }

    /// Get completions for the current context
    pub fn get_completions(&self, context: &CompletionContext) -> Vec<Suggestion> {
        let prefix = context.get_prefix();

        // If we're typing a command name
        if let Some(command) = context.parse_command() {
            // Check if we're after a command that expects arguments
            let input_parts: Vec<&str> = prefix.split_whitespace().collect();

            if input_parts.len() == 1 && prefix.ends_with(command) {
                // We're still typing the command name
                self.complete_command(prefix.trim_start_matches('/'))
            } else if input_parts.len() >= 2 {
                // We're typing arguments
                self.complete_arguments(command, prefix)
            } else {
                Vec::new()
            }
        } else if prefix.starts_with('/') {
            // We're typing a command name
            self.complete_command(prefix.trim_start_matches('/'))
        } else {
            // We're typing free text - show history
            self.complete_history(prefix)
        }
    }

    /// Complete slash command names
    fn complete_command(&self, prefix: &str) -> Vec<Suggestion> {
        // If prefix is empty (just typed "/"), show limited commands
        let prefix_to_match = if prefix.is_empty() { "" } else { prefix };

        let mut results: Vec<_> = self
            .specs
            .iter()
            .filter(|spec| {
                spec.name.starts_with(prefix_to_match)
                    || spec
                        .aliases
                        .iter()
                        .any(|alias| alias.starts_with(prefix_to_match))
            })
            .map(|spec| Suggestion {
                display: format!("/{}", spec.name),
                replacement: format!("/{} ", spec.name),
                description: Some(spec.summary.to_string()),
                kind: SuggestionKind::Command,
            })
            .collect();

        // Limit to 10 results when prefix is empty to avoid overwhelming user
        if prefix.is_empty() && results.len() > 10 {
            results.truncate(10);
        }

        results
    }

    /// Complete argument values based on command
    fn complete_arguments(&self, command: &str, prefix: &str) -> Vec<Suggestion> {
        let parts: Vec<&str> = prefix.split_whitespace().collect();
        let last_word = parts.last().unwrap_or(&"");

        match command {
            "/model" | "/m" => self.complete_models(last_word),
            "/mode" | "/permissions" | "/perm" => self.complete_modes(last_word),
            "/session" => self.complete_session_actions(last_word),
            "/theme" => self.complete_themes(last_word),
            _ => Vec::new(),
        }
    }

    /// Complete model names
    fn complete_models(&self, prefix: &str) -> Vec<Suggestion> {
        self.models
            .iter()
            .filter(|model| model.starts_with(prefix))
            .map(|model| Suggestion {
                display: model.clone(),
                replacement: format!("{model} "),
                description: None,
                kind: SuggestionKind::ModelName,
            })
            .collect()
    }

    /// Complete permission modes
    fn complete_modes(&self, prefix: &str) -> Vec<Suggestion> {
        let modes = vec![
            ("read-only", "No filesystem writes"),
            ("workspace-write", "Writes within workspace only"),
            ("danger-full-access", "Unrestricted access"),
        ];

        modes
            .into_iter()
            .filter(|(name, _)| name.starts_with(prefix))
            .map(|(name, desc)| Suggestion {
                display: name.to_string(),
                replacement: format!("{name} "),
                description: Some(desc.to_string()),
                kind: SuggestionKind::PermissionMode,
            })
            .collect()
    }

    /// Complete session actions
    fn complete_session_actions(&self, prefix: &str) -> Vec<Suggestion> {
        let actions = vec![
            ("list", "List all sessions"),
            ("switch", "Switch to a session"),
            ("fork", "Create a new session branch"),
        ];

        actions
            .into_iter()
            .filter(|(name, _)| name.starts_with(prefix))
            .map(|(name, desc)| Suggestion {
                display: name.to_string(),
                replacement: format!("{name} "),
                description: Some(desc.to_string()),
                kind: SuggestionKind::Argument,
            })
            .collect()
    }

    /// Complete theme names
    fn complete_themes(&self, prefix: &str) -> Vec<Suggestion> {
        self.themes
            .iter()
            .filter(|theme| theme.starts_with(prefix))
            .map(|theme| Suggestion {
                display: theme.clone(),
                replacement: format!("{theme} "),
                description: None,
                kind: SuggestionKind::ThemeName,
            })
            .collect()
    }

    /// Complete from command history
    fn complete_history(&self, prefix: &str) -> Vec<Suggestion> {
        if prefix.is_empty() {
            return Vec::new();
        }

        self.history
            .iter()
            .filter(|entry| entry.starts_with(prefix) || entry.contains(prefix))
            .map(|entry| Suggestion {
                display: entry.clone(),
                replacement: entry.clone(),
                description: None,
                kind: SuggestionKind::HistoryEntry,
            })
            .collect()
    }
}

impl Default for CompletionEngine {
    fn default() -> Self {
        Self::new()
    }
}
