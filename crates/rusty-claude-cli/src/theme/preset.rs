use crate::theme::{SemanticColors, TerminalCapability};

use crossterm::style::Color;

/// Built-in theme presets
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemePreset {
    /// Professional terminal (blue/green semantic colors)
    Default,
    /// WCAG AAA compliant high contrast
    HighContrast,
    /// Basic 16-color compatibility
    Legacy,
    /// Monochrome (ASCII only, no colors)
    Monochrome,
}

impl ThemePreset {
    /// Get the semantic colors for this preset at full capability
    fn semantic_colors(&self) -> SemanticColors {
        match self {
            Self::Default => SemanticColors {
                // Moss for successful work.
                success: Color::Rgb {
                    r: 138,
                    g: 174,
                    b: 106,
                },
                // Clay for failures.
                error: Color::Rgb {
                    r: 217,
                    g: 107,
                    b: 95,
                },
                // Ember for warnings.
                warning: Color::Rgb {
                    r: 230,
                    g: 162,
                    b: 60,
                },
                // Slate for metadata.
                muted: Color::Rgb {
                    r: 125,
                    g: 135,
                    b: 144,
                },
                // Ochre for Oog identity and active controls.
                primary: Color::Rgb {
                    r: 215,
                    g: 168,
                    b: 62,
                },
                // Bone for titles and high-value information.
                secondary: Color::Rgb {
                    r: 232,
                    g: 225,
                    b: 207,
                },
                // Dark ochre for quiet structure.
                border: Color::Rgb {
                    r: 104,
                    g: 81,
                    b: 42,
                },
            },
            Self::HighContrast => SemanticColors {
                // Pure green for maximum contrast
                success: Color::Rgb { r: 0, g: 255, b: 0 },
                // Pure red for maximum contrast
                error: Color::Rgb { r: 255, g: 0, b: 0 },
                // Pure yellow for maximum contrast
                warning: Color::Rgb {
                    r: 255,
                    g: 255,
                    b: 0,
                },
                // Light gray for metadata on dark backgrounds
                muted: Color::Rgb {
                    r: 192,
                    g: 192,
                    b: 192,
                },
                // High contrast blue
                primary: Color::Rgb {
                    r: 0,
                    g: 191,
                    b: 255,
                },
                secondary: Color::Rgb {
                    r: 135,
                    g: 206,
                    b: 250,
                },
                border: Color::Rgb {
                    r: 0,
                    g: 119,
                    b: 190,
                },
            },
            Self::Legacy => SemanticColors {
                // Green (ANSI)
                success: Color::Green,
                // Red (ANSI)
                error: Color::Red,
                // Yellow (ANSI)
                warning: Color::Yellow,
                // Dark gray (ANSI)
                muted: Color::DarkGrey,
                // Blue (ANSI)
                primary: Color::Blue,
                // Cyan (ANSI)
                secondary: Color::Cyan,
                // Dark blue (ANSI)
                border: Color::DarkBlue,
            },
            Self::Monochrome => SemanticColors {
                success: Color::Grey,
                error: Color::DarkGrey,
                warning: Color::Grey,
                muted: Color::DarkGrey,
                primary: Color::Grey,
                secondary: Color::White,
                border: Color::DarkGrey,
            },
        }
    }

    /// Degrade colors based on terminal capability
    pub fn degrade_for_capability(&self, capability: &TerminalCapability) -> SemanticColors {
        let colors = self.semantic_colors();

        match capability {
            TerminalCapability::TrueColor => colors,
            TerminalCapability::EightBit => {
                // Convert RGB to 256-color palette
                Self::rgb_to_256(colors)
            }
            TerminalCapability::Ansi => {
                // Convert RGB to 16 ANSI colors
                Self::rgb_to_ansi(colors)
            }
            TerminalCapability::Monochrome => SemanticColors {
                success: Color::Grey,
                error: Color::DarkGrey,
                warning: Color::Grey,
                muted: Color::DarkGrey,
                primary: Color::Grey,
                secondary: Color::White,
                border: Color::DarkGrey,
            },
        }
    }

    /// Convert RGB colors to 256-color palette
    fn rgb_to_256(colors: SemanticColors) -> SemanticColors {
        fn convert(color: Color) -> Color {
            match color {
                Color::Rgb { r, g, b } => {
                    // Simple RGB to 256-color conversion
                    // This is a simplified version - a proper implementation would use
                    // the full 6x6x6 color cube + grayscale
                    let r = (f32::from(r) / 255.0 * 5.0).round() as u8;
                    let g = (f32::from(g) / 255.0 * 5.0).round() as u8;
                    let b = (f32::from(b) / 255.0 * 5.0).round() as u8;
                    Color::AnsiValue(16 + 36 * r + 6 * g + b)
                }
                other => other,
            }
        }

        SemanticColors {
            success: convert(colors.success),
            error: convert(colors.error),
            warning: convert(colors.warning),
            muted: convert(colors.muted),
            primary: convert(colors.primary),
            secondary: convert(colors.secondary),
            border: convert(colors.border),
        }
    }

    /// Convert RGB colors to 16 ANSI colors
    fn rgb_to_ansi(colors: SemanticColors) -> SemanticColors {
        fn convert(color: Color) -> Color {
            match color {
                Color::Rgb { r, g, b } => {
                    // Simple RGB to ANSI conversion
                    // Find the strongest channel first, then check for bright colors
                    if r.abs_diff(g) < 25 && g.abs_diff(b) < 25 {
                        if r >= 160 {
                            Color::White
                        } else {
                            Color::Grey
                        }
                    } else if b >= 120 && r >= 60 && r > g {
                        Color::Magenta
                    } else if r >= 150 && g >= 100 && b < 120 {
                        Color::Yellow
                    } else if b >= g && b >= r {
                        // Blue is strongest (or tied for strongest)
                        if b > 200 && g > 200 && r < 100 {
                            Color::Cyan
                        } else {
                            Color::Blue
                        }
                    } else if g >= r && g >= b {
                        // Green is strongest (or tied for strongest)
                        if r > 200 && g > 200 && b < 100 {
                            Color::Yellow
                        } else if r < 160 && g > 120 && b < 120 {
                            Color::Green
                        } else {
                            Color::DarkGreen
                        }
                    } else {
                        // Red is strongest
                        if r > 200 && g < 100 && b < 100 {
                            Color::Red
                        } else {
                            Color::DarkRed
                        }
                    }
                }
                other => other,
            }
        }

        SemanticColors {
            success: convert(colors.success),
            error: convert(colors.error),
            warning: convert(colors.warning),
            muted: convert(colors.muted),
            primary: convert(colors.primary),
            secondary: convert(colors.secondary),
            border: convert(colors.border),
        }
    }
}

/// Load theme from preset name string
pub fn load_theme(name: &str) -> Option<ThemePreset> {
    match name.to_lowercase().as_str() {
        "default" => Some(ThemePreset::Default),
        "high-contrast" | "highcontrast" => Some(ThemePreset::HighContrast),
        "legacy" => Some(ThemePreset::Legacy),
        "monochrome" => Some(ThemePreset::Monochrome),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_parsing() {
        assert_eq!(load_theme("default"), Some(ThemePreset::Default));
        assert_eq!(load_theme("DEFAULT"), Some(ThemePreset::Default));
        assert_eq!(load_theme("high-contrast"), Some(ThemePreset::HighContrast));
        assert_eq!(load_theme("legacy"), Some(ThemePreset::Legacy));
        assert_eq!(load_theme("monochrome"), Some(ThemePreset::Monochrome));
        assert_eq!(load_theme("unknown"), None);
    }

    #[test]
    fn test_degradation_to_ansi() {
        let preset = ThemePreset::Default;
        let colors = preset.degrade_for_capability(&TerminalCapability::Ansi);

        // After degradation, should be ANSI colors
        match colors.success {
            Color::Green => {}
            _ => panic!("Expected green for success in ANSI mode"),
        }
    }

    #[test]
    fn ansi_uses_white_for_bone() {
        let colors = ThemePreset::Default.degrade_for_capability(&TerminalCapability::Ansi);
        assert_eq!(colors.secondary, Color::White);
    }

    #[test]
    fn test_degradation_to_monochrome() {
        let preset = ThemePreset::Default;
        let colors = preset.degrade_for_capability(&TerminalCapability::Monochrome);

        // In monochrome, everything should be gray
        match colors.success {
            Color::Grey => {}
            _ => panic!("Expected grey for success in monochrome mode"),
        }
    }
}
