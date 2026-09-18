# Oog Code Design System Reference

**Purpose**: This document serves as the single source of truth for all colors, emojis, and design decisions for Oog Code. All development should reference this document to ensure consistency.

---

## Color Palette: Obsidian + Ochre Theme

### Primary Brand Colors

| Color Name | RGB Values | Hex | Usage | ANSI Color Code |
|------------|-----------|-----|-------|-----------------|
| **Ochre (Primary)** | `RGB(215, 168, 62)` | `#D7A83E` | Brand, ASCII art, active controls | `\x1b[38;2;215;168;62m` |
| **Bone (Secondary)** | `RGB(232, 225, 207)` | `#E8E1CF` | Titles and high-value information | `\x1b[38;2;232;225;207m` |
| **Dark Ochre (Border)** | `RGB(104, 81, 42)` | `#68512A` | Subtle borders and dividers | `\x1b[38;2;104;81;42m` |

### Semantic Colors (Status & Feedback)

| Color Name | RGB Values | Usage | When to Use |
|------------|-----------|-------|------------|
| **Success Green** | `RGB(76, 175, 80)` | Success states, completed actions, positive feedback | ✝️ completion, successful operations |
| **Error Red** | `RGB(244, 67, 54)` | Errors, failures, blocked actions, negative feedback | Error messages, failures |
| **Warning Yellow** | `RGB(255, 193, 7)` | Warnings, cautions, important notices | Warnings, alerts |
| **Muted Gray** | `RGB(128, 128, 128)` | Metadata, timestamps, secondary info, labels | Labels, metadata text |

### Color-Coded Bullet Points (●)

| Bullet Color | RGB Value | Usage Context |
|--------------|-----------|----------------|
| **White/Default** | Terminal default | Neutral info, general lists |
| **Light Purple** | `RGB(189, 147, 249)` | User actions, user commands, user data |
| **Success Green** | `RGB(76, 175, 80)` | Success messages, completed tasks, positive states |
| **Warning Yellow** | `RGB(255, 193, 7)` | Warnings, cautions, pending states |
| **Error Red** | `RGB(244, 67, 54)` | Errors, failures, critical issues |

---

## Emoji System

### ✅ Approved Emojis (Use These)

| Emoji | Context | Usage Examples |
|-------|---------|----------------|
| 🪨 | **Primary brand icon** | Oog identity, welcome screen signature |
| ✝️ | Success, completion | Task completion, successful operations |
| 🙏 | Tips, suggestions | Help tips, suggestions, guidance |
| 🍞 | Contextual (religious) | When appropriate for religious context |
| ⛪ | Contextual (religious) | When appropriate for religious context |

### ❌ Blocked Emojis (Do NOT Use in UI)

These emojis should **NOT** appear in the Oog Code UI elements. They may appear in AI responses unless `MINSEO_DISABLE_EMOJIS` is set for legacy compatibility.

**Decorative Emojis to Avoid in UI:**
- 🎨, 💅, 🍅, 🚀, ✨, 🎉, 💡 (use 🙏 instead)
- Any emoji not in the "Approved" list above

### Emoji Control

**Environment Variable**: `MINSEO_DISABLE_EMOJIS`
- **When set** (any value): All emojis are stripped from AI responses
- **When not set**: AI can use emojis (UI always uses approved emojis only)

**Implementation**: The emoji stripping function in `render.rs` filters out decorative emojis while preserving text symbols and approved UI emojis.

---

## Design Patterns

### Welcome Screen

**Layout**: Clean, no borders
```
[Light Purple ASCII Art]
"by Troglodytic 🪨"

[Empty line]

● Model: [value]
● Permissions: [value]
● Branch: [value]
● Workspace: [value]
● Directory: [value]

[Empty line]

[Pale Purple Tip with 🙏]

[Empty line]

[Muted Gray: Recent activity]

[Empty line]
```

**Color Rules**:
- ASCII art: **Light Purple** (`RGB(189, 147, 249)`)
- Bullets (●): **Light Purple** for user data
- Labels (Model, Permissions, etc.): **Muted Gray**
- Values: Default terminal color
- Tips: **Pale Purple**
- Activity text: **Muted Gray**

### REPL Message Headers

**User Messages**:
```
You: (in Light Purple)
[newline]
[content]
[newline]
```

**AI/Oog Messages**:
```
🪨 Oog: (in Light Purple)
[newline]
[content]
[newline]
```

**Color Rules**:
- User header: **Light Purple** (`RGB(189, 147, 249)`)
- AI header: **Light Purple** (`RGB(189, 147, 249)`) with 🧔🏻‍♀️ icon
- No divider lines - clean format with just the name in theme color followed by colon
- Spacing: Empty line before header and after content

### Width Constraints

- **Max content width**: 75% of terminal width
- **Absolute maximum**: 100 characters
- **Absolute minimum**: 60 characters
- **Purpose**: Better readability on wide screens

---

## Implementation Guidelines

### 1. Always Reference This Document

Before adding colors or emojis:
1. Check this document for the correct color
2. Use the exact RGB values specified
3. Only use emojis from the "Approved" list
4. Never hardcode colors - use the theme system

### 2. Theme System Usage

**In Rust code**:
```rust
use theme::{Theme, ThemePreset};

let theme = Theme::new(ThemePreset::Default);
let primary = theme.semantic.primary;     // Light Purple
let secondary = theme.semantic.secondary; // Pale Purple
let border = theme.semantic.border;       // Medium Purple
let success = theme.semantic.success;     // Green
let error = theme.semantic.error;         // Red
let warning = theme.semantic.warning;     // Yellow
let muted = theme.semantic.muted;         // Gray
```

**Convert to ANSI**:
```rust
let ansi_color = color_to_ansi(theme.semantic.primary);
// Returns: "\x1b[38;2;189;147;249m"
```

### 3. When to Use Specific Colors

| Situation | Color | Example |
|-----------|-------|---------|
| Brand identity, headers | Light Purple | ASCII art, main titles |
| Highlights, tips, welcomes | Pale Purple | Tips, welcome messages |
| User data, commands | Light Purple | Metadata bullets |
| Success/completion | Green | ✝️ operations, success messages |
| Errors | Red | Error messages |
| Warnings | Yellow | Warning messages |
| Labels, metadata | Muted Gray | "Model:", "Permissions:" labels |
| User actions | Light Purple | User commands, user-generated content |

---

## File Locations

### Theme System Files
- `crates/rusty-claude-cli/src/theme/mod.rs` - Theme structure
- `crates/rusty-claude-cli/src/theme/preset.rs` - Color definitions
- `crates/rusty-claude-cli/src/render.rs` - Color rendering utilities

### Layout Files
- `crates/rusty-claude-cli/src/layout.rs` - Layout utility functions

### UI Implementation Files
- `crates/rusty-claude-cli/src/main.rs` - Welcome screen, REPL, main UI
- `crates/rusty-claude-cli/src/app.rs` - AI response rendering

---

## Common Mistakes to Avoid

1. ❌ **Hardcoding colors** - Always use the theme system
2. ❌ **Using unapproved emojis in UI** - Only use the 5 approved emojis
3. ❌ **Using green for brand elements** - Use Light Purple
4. ❌ **Forgetting to set width constraints** - Always use 75% width
5. ❌ **Adding emoji in user-facing UI text** - Keep UI clean, only AI uses emojis (when enabled)

---

## Quick Reference: Color Decision Tree

```
What are you coloring?
├── Brand/Identity/Header?
│   └── Use: Light Purple (RGB: 189, 147, 249)
├── Welcome/Highlight/Accent?
│   └── Use: Pale Purple (RGB: 219, 192, 254)
├── Success/Completion?
│   └── Use: Green (RGB: 76, 175, 80) with ✝️
├── Error/Failure?
│   └── Use: Red (RGB: 244, 67, 54)
├── Warning?
│   └── Use: Yellow (RGB: 255, 193, 7)
├── Label/Metadata?
│   └── Use: Muted Gray (RGB: 128, 128, 128)
└── User Action/Command?
    └── Use: Light Purple (RGB: 189, 147, 249) with ●
```

---

## Version History

| Date | Changes | Author |
|------|---------|--------|
| 2026-04-06 | Updated theme color from Cerulean Blue to Light Purple (RGB: 189, 147, 249) | Claude Code |
| 2026-04-06 | Initial design system document with Cerulean Blue theme and emoji guidelines | Claude Code |
