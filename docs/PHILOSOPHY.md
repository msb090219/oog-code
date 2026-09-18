# Oog Code Philosophy

**Simple outside. Powerful inside.**

---

## The Core Principle

Oog Code follows the Apple design philosophy: **deceptively simple interfaces backed by sophisticated engineering**.

### What This Means

**Outside (What users see):**
- Clean, minimal interface
- Intuitive interactions
- No overwhelming options
- Beautiful typography and spacing

**Inside (What makes it work):**
- Intelligent defaults
- Adaptive behavior
- Progressive disclosure
- Powerful features that activate when needed

---

## Examples in Action

### Status Bar

**What users see:**
```
opus · 1234 tokens · $0.0123
```

**What's happening inside:**
- Detects terminal width
- Prioritizes information (model > tokens > cost > permissions > git > session)
- Automatically drops less important elements on narrow terminals
- Shortens model names intelligently (claude-opus-4-6 → opus)
- Shows icons only when there's space

**Result**: Always looks clean, regardless of terminal size.

### Output Layer

**What users see:**
```
✓ ReadFile
✓ Bash
Beautiful markdown rendering with syntax highlighting.
```

**What's happening inside:**
- Collapsible sections (user can collapse tool output)
- Output modes (minimal/normal/verbose for progressive disclosure)
- Smart markdown parsing with syntax detection
- Text wrapping that respects terminal width
- Scroll offset management
- ANSI color codes for beautiful typography

**Result**: Powerful features don't clutter the interface.

### Event Loop

**What users see:**
- Smooth 60fps rendering
- Responsive UI during streaming
- No lag or jank

**What's happening inside:**
- Tokio::select! multiplexing
- mpsc channel event distribution
- Render throttling (max 60fps)
- Event coalescing
- Async I/O throughout

**Result**: Feels magical and smooth, powered by sophisticated async architecture.

---

## Design Guidelines

### 1. Default to Minimal

**✅ Good:**
```rust
// OutputLayer defaults to Minimal mode
impl Default for OutputLayer {
    fn default() -> Self {
        Self::new(OutputMode::Minimal, CollapseThresholds::default(), 12)
    }
}
```

**❌ Bad:**
```rust
// Don't overwhelm users with everything enabled by default
impl Default for OutputLayer {
    fn default() -> Self {
        Self::new(OutputMode::Verbose, CollapseThresholds::show_all(), 100)
    }
}
```

### 2. Progressive Disclosure

Show what's important. Hide what's advanced.

- **Minimal mode**: Hide successful tool calls, hide section headers
- **Normal mode**: Show section headers, compact tool output
- **Verbose mode**: Show everything (for debugging)

### 3. Adaptive Behavior

The system should adapt to the environment, not force the user to adapt.

- Terminal width changes? Status bar adapts automatically.
- User resizes terminal? Output reflows intelligently.
- Color terminal not available? Graceful fallback to monochrome.

### 4. Beautiful Defaults

Users shouldn't need to configure to get a great experience.

- Themes come pre-configured (dark, light, solarized, catppuccin)
- Color capability detection is automatic
- Spacing and typography are hand-tuned

---

## Implementation Examples

### Complex Feature, Simple Interface

**Collapsible Sections:**

```rust
// Inside: Complex state management
pub struct OutputLayer {
    collapsed_sections: Vec<String>,
    // ... other fields
}

// Outside: Simple toggle API
impl OutputLayer {
    pub fn toggle_section(&mut self, section_id: &str) {
        // Power user feature, but simple interface
    }
}
```

**Smart Truncation:**

```rust
// Inside: Priority-based algorithm
while !components.is_empty() && status_text.len() > available_width {
    components.pop(); // Drop lowest priority first
}

// Outside: Just works on any terminal width
```

---

## Checklist for New Features

When adding features to Oog Code, ask:

- [ ] **Is the interface simple?** (Users understand it immediately)
- [ ] **Is the implementation powerful?** (Handles edge cases intelligently)
- [ ] **Does it adapt?** (Works in different contexts)
- [ ] **Are the defaults beautiful?** (No configuration needed for great UX)
- [ ] **Is complexity hidden?** (Power features don't clutter the interface)

---

## Summary

> "Make it simple, but not simpler." - Albert Einstein

Oog Code aims to be **powerfully simple**. Every feature should feel effortless to use while being backed by sophisticated engineering that handles complexity gracefully.

**Simple outside, powerful inside.**

---

*Last Updated: 2026-04-25*
