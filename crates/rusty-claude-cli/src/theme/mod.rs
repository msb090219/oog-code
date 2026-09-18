#[allow(clippy::trivially_copy_pass_by_ref)]
pub mod capability;
#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::trivially_copy_pass_by_ref
)]
pub mod preset;

use crossterm::style::Color;
use std::env;

pub use capability::{detect_capability, TerminalCapability};
pub use preset::{load_theme, ThemePreset};

/// Semantic color tokens for consistent theming
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemanticColors {
    /// Success states, completed actions
    pub success: Color,
    /// Errors, failures, blocked actions
    pub error: Color,
    /// Warnings, cautions
    pub warning: Color,
    /// Metadata, timestamps, secondary info
    pub muted: Color,
    /// Primary UI color (Ochre)
    pub primary: Color,
    /// Secondary accent color (Bone)
    pub secondary: Color,
    /// Border color (Dark Ochre)
    pub border: Color,
}

/// Spacing constants following 4pt rhythm
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Spacing {
    /// Section separator (2 newlines)
    pub section: usize,
    /// Subsection (1 newline)
    pub subsection: usize,
    /// List items (0 newlines, indent only)
    pub list: usize,
    /// Code block padding (1 newline above/below)
    pub code: usize,
}

impl Default for Spacing {
    fn default() -> Self {
        Self {
            section: 2,
            subsection: 1,
            list: 0,
            code: 1,
        }
    }
}

/// Complete theme configuration
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Theme {
    /// Semantic color tokens
    pub semantic: SemanticColors,
    /// Spacing following 4pt rhythm
    pub spacing: Spacing,
    /// Terminal capability level
    pub capability: TerminalCapability,
}

impl Theme {
    /// Create a new theme with automatic capability detection
    pub fn new(preset: ThemePreset) -> Self {
        let capability = detect_capability();

        // Apply capability degradation to preset colors if needed
        let semantic = preset.degrade_for_capability(&capability);

        Self {
            semantic,
            spacing: Spacing::default(),
            capability,
        }
    }

    /// Check if `NO_COLOR` is set
    pub fn is_no_color() -> bool {
        env::var("NO_COLOR").is_ok()
    }

    /// Create a monochrome theme (respects `NO_COLOR`)
    pub fn monochrome() -> Self {
        Self {
            semantic: SemanticColors {
                success: Color::Grey,
                error: Color::DarkGrey,
                warning: Color::Grey,
                muted: Color::DarkGrey,
                primary: Color::Grey,
                secondary: Color::White,
                border: Color::DarkGrey,
            },
            spacing: Spacing::default(),
            capability: TerminalCapability::Monochrome,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spacing_defaults() {
        let spacing = Spacing::default();
        assert_eq!(spacing.section, 2);
        assert_eq!(spacing.subsection, 1);
        assert_eq!(spacing.list, 0);
        assert_eq!(spacing.code, 1);
    }

    #[test]
    fn test_theme_creation() {
        let theme = Theme::new(ThemePreset::Default);
        assert_eq!(theme.spacing, Spacing::default());
    }
}
