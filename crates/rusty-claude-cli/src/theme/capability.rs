use crossterm::style::Color;
use std::env;

/// Terminal color capability levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TerminalCapability {
    /// No color support
    Monochrome,
    /// Basic 16 ANSI colors
    Ansi,
    /// 256-color palette
    EightBit,
    /// 24-bit true color support
    TrueColor,
}

impl TerminalCapability {
    /// Check if this capability supports RGB colors
    pub fn supports_rgb(&self) -> bool {
        matches!(self, Self::TrueColor | Self::EightBit)
    }

    /// Check if this capability supports any color
    pub fn supports_color(&self) -> bool {
        !matches!(self, Self::Monochrome)
    }
}

/// Detect terminal color capability
pub fn detect_capability() -> TerminalCapability {
    // Check NO_COLOR environment variable (https://no-color.org/)
    if env::var("NO_COLOR").is_ok() {
        return TerminalCapability::Monochrome;
    }

    // Check COLORTERM environment variable
    if let Ok(colorterm) = env::var("COLORTERM") {
        if colorterm.contains("truecolor") || colorterm.contains("24bit") {
            return TerminalCapability::TrueColor;
        }
    }

    // Check TERM environment variable
    if let Ok(term) = env::var("TERM") {
        // Check for 24-bit color support
        if term.contains("24bit") || term.contains("truecolor") {
            return TerminalCapability::TrueColor;
        }

        // Check for 256-color support
        if term.contains("256color") {
            return TerminalCapability::EightBit;
        }

        // Check for dtterm (supports 256 colors)
        if term.contains("xterm") || term.contains("dtterm") {
            return TerminalCapability::EightBit;
        }

        // Check for basic ANSI color
        if term.contains("color") || term.contains("ansi") || term.contains("linux") {
            return TerminalCapability::Ansi;
        }

        // Dumb terminal
        if term == "dumb" {
            return TerminalCapability::Monochrome;
        }
    }

    // Default to ANSI for unknown terminals
    TerminalCapability::Ansi
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truecolor_supports_rgb() {
        assert!(TerminalCapability::TrueColor.supports_rgb());
        assert!(TerminalCapability::EightBit.supports_rgb());
        assert!(!TerminalCapability::Ansi.supports_rgb());
        assert!(!TerminalCapability::Monochrome.supports_rgb());
    }

    #[test]
    fn test_capability_ordering() {
        assert!(TerminalCapability::TrueColor > TerminalCapability::EightBit);
        assert!(TerminalCapability::EightBit > TerminalCapability::Ansi);
        assert!(TerminalCapability::Ansi > TerminalCapability::Monochrome);
    }
}
