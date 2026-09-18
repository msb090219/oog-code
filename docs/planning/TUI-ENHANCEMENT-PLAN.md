# Retired TUI Enhancement Plan — Claw Code (`rusty-claude-cli`)

> **Retired.** The project intentionally returned to the original inline
> terminal interface. Do not implement this proposal, add ratatui, or create a
> TUI module. The active plan is [CLI-ROADMAP.md](CLI-ROADMAP.md). This file is
> retained solely as historical context.

## Executive Summary

This plan covers a comprehensive analysis of the current terminal user interface and proposes phased enhancements that will transform the existing REPL/prompt CLI into a polished, modern TUI experience with a **persistent async input composer** — while preserving the existing clean architecture and test coverage.

### Architecture Vision

**Hybrid Layered Renderer**: A unified rendering system supporting both inline mode (default) and full-screen TUI (opt-in), built on ratatui from Phase 0. The system uses:
- **Custom event loop** with tokio::select! for multiplexing stream events, input handling, and render ticks
- **Event-based architecture** where stream handlers send render events via mpsc channels to the UI task
- **Throttled event-driven rendering** (max 60fps with event coalescing) for smooth animations
- **Persistent input composer** with non-blocking input during streaming
- **Layered terminal rendering**: output layer (scrolling), input layer (fixed bottom), overlay layer (suggestions)

### Key Technical Decisions

| Area | Decision | Rationale |
|------|----------|-----------|
| **UI Framework** | Adopt ratatui from Phase 0 | Provides layout primitives and terminal management; custom renderer bypasses widget trait for streaming compatibility |
| **Input System** | Custom input handler | Replaces rustyline's terminal ownership while maintaining UX quality; launches minimal but competent feature set |
| **State Sync** | Event-based via mpsc channels | Stream handlers send events to UI task; clean separation of concerns |
| **Render Strategy** | Throttled event-driven (60fps max) | Balances responsiveness with efficiency; coalesces rapid events |
| **Streaming Delay** | Remove entirely | Proper incremental rendering makes artificial delay unnecessary |
| **Markdown Parsing** | Block buffer with boundary detection | Completed blocks cached; active block provisionally rendered with timeout fallback |
| **Tool Expansion** | Live expansion with viewport | Scrollable truncated window updates as lines arrive |
| **Queue Persistence** | Append-only journal | Separate .queue file with atomic writes via rename; replay on load |
| **Interruption** | Two-stage Ctrl+C | First press cancels current generation, second exits app |
| **Terminal Resize** | Reflow visible only | Store content in layout-independent form; reflow only visible region on resize |
| **Syntax Highlighting** | Language detection heuristics | Fast path for known languages; fallback to plain text for unknown/large outputs |
| **Diff Visualization** | Word-level diffs | Precise change detection within lines |
| **Testing Strategy** | Layered approach | Unit tests + mock terminal snapshots + PTY integration tests |
| **Platform Support** | Full cross-platform | Linux, macOS, Windows parity from day one |
| **Color Capability** | Detection-based | Probe terminal capabilities with graceful fallback |
| **Session Safety** | PID file + session management | Detect concurrent instances; offer to attach or start new session |

---

## 1. Current Architecture Analysis

### Crate Map

| Crate | Purpose | Lines | TUI Relevance |
|---|---|---|---|
| `rusty-claude-cli` | Main binary: REPL loop, arg parsing, rendering, API bridge | ~3,600 | **Primary TUI surface** |
| `runtime` | Session, conversation loop, config, permissions, compaction | ~5,300 | Provides data/state |
| `api` | Anthropic HTTP client + SSE streaming | ~1,500 | Provides stream events |
| `commands` | Slash command metadata/parsing/help | ~470 | Drives command dispatch |
| `tools` | 18 built-in tool implementations | ~3,500 | Tool execution display |

### Current TUI Components

| Component | File | What It Does Today | Quality |
|---|---|---|---|
| **Input** | `input.rs` (269 lines) | `rustyline`-based line editor with slash-command tab completion, Shift+Enter newline, history | ✅ Solid |
| **Rendering** | `render.rs` (641 lines) | Markdown→terminal rendering (headings, lists, tables, code blocks with syntect highlighting, blockquotes), spinner widget | ✅ Good |
| **App/REPL loop** | `main.rs` (3,159 lines) | The monolithic `LiveCli` struct: REPL loop, all slash command handlers, streaming output, tool call display, permission prompting, session management | ⚠️ Monolithic |
| **Alt App** | `app.rs` (398 lines) | An earlier `CliApp` prototype with `ConversationClient`, stream event handling, `TerminalRenderer`, output format support | ⚠️ Appears unused/legacy |

### Key Dependencies

- **crossterm 0.28** — terminal control (cursor, colors, clear)
- **pulldown-cmark 0.13** — Markdown parsing
- **syntect 5** — syntax highlighting
- **rustyline 15** — line editing with completion
- **serde_json** — tool I/O formatting

### Strengths

1. **Clean rendering pipeline**: Markdown rendering is well-structured with state tracking, table rendering, code highlighting
2. **Rich tool display**: Tool calls get box-drawing borders (`╭─ name ─╮`), results show ✓/✗ icons
3. **Comprehensive slash commands**: 15 commands covering model switching, permissions, sessions, config, diff, export
4. **Session management**: Full persistence, resume, list, switch, compaction
5. **Permission prompting**: Interactive Y/N approval for restricted tool calls
6. **Thorough tests**: Every formatting function, every parse path has unit tests

### Weaknesses & Gaps

1. **`main.rs` is a 3,159-line monolith** — all REPL logic, formatting, API bridging, session management, and tests in one file
2. **No alternate-screen / full-screen layout** — everything is inline scrolling output
3. **No progress bars** — only a single braille spinner; no indication of streaming progress or token counts during generation
4. **No visual diff rendering** — `/diff` just dumps raw git diff text
5. **No syntax highlighting in streamed output** — markdown rendering only applies to tool results, not to the main assistant response stream
6. **No status bar / HUD** — model, tokens, session info not visible during interaction
7. **No image/attachment preview** — `SendUserMessage` resolves attachments but never displays them
8. **Streaming is char-by-char with artificial delay** — `stream_markdown` sleeps 8ms per whitespace-delimited chunk
9. **Limited color theme customization** — Uses `Theme::new(ThemePreset::Default)` with Light Purple (RGB: 189, 147, 249) theme; see DESIGN_SYSTEM.md for full color palette
10. **No resize handling** — no terminal size awareness for wrapping, truncation, or layout
11. **Dual app structs** — `app.rs` has a separate `CliApp` that duplicates `LiveCli` from `main.rs`
12. **No pager for long outputs** — `/status`, `/config`, `/memory` can overflow the viewport
13. **Tool results not collapsible** — large bash outputs flood the screen
14. **No thinking/reasoning indicator** — when the model is in "thinking" mode, no visual distinction
15. **No auto-complete for tool arguments** — only slash command names complete

---

## 2. Enhancement Plan

### Phase 0: Structural Cleanup + Foundation (Week 1-2)

**Goal**: Break the monolith, remove dead code, establish the module structure for TUI work, and adopt ratatui.

| Task | Description | Effort | Dependencies |
|---|---|---|---|
| 0.1 | **Extract `LiveCli` into `app.rs`** — Move the entire `LiveCli` struct, its impl, and helpers (`format_*`, `render_*`, session management) out of `main.rs` into focused modules: `app.rs` (core), `format.rs` (report formatting), `session_manager.rs` (session CRUD) | M | - |
| 0.2 | **Remove or merge the legacy `CliApp`** — The existing `app.rs` has an unused `CliApp` with its own `ConversationClient`-based rendering. Delete it; merge unique features (stream event handler pattern) into the active `LiveCli` | S | 0.1 |
| 0.3 | **Extract `main.rs` arg parsing** — Consolidate on the hand-rolled parser (more feature-complete) and move to `args.rs` | S | - |
| 0.4 | **Create a `tui/` module** — Introduce `crates/rusty-claude-cli/src/tui/mod.rs` as the namespace for all TUI components | S | - |
| 0.5 | **Add ratatui dependency** — Add `ratatui = "0.28"` as a core dependency (not feature-gated). Import terminal backend and layout primitives | S | - |
| 0.6 | **Design custom renderer trait** — Define `LayeredRenderer` trait that uses ratatui's terminal backend but bypasses the widget trait for streaming compatibility. Supports three rendering layers: output, input, overlay | M | 0.5 |
| 0.7 | **Event types definitions** — Define `RenderEvent` enum (TextDelta, ToolStart, ToolProgress, ToolComplete, UsageUpdate, etc.) and `UiState` struct | M | - |
| 0.8 | **Custom event loop skeleton** — Implement `EventLoop` struct with tokio::select! multiplexing stream events, input events, and render ticks. Set up mpsc channels for event distribution | L | 0.6, 0.7 |

### Phase 1: Status Bar & Live HUD (Week 3)

**Goal**: Persistent information display during interaction.

| Task | Description | Effort | Dependencies |
|---|---|---|---|
| 1.1 | **Terminal-size-aware status line** — Use ratatui's layout system to render a bottom-pinned status bar. Fixed-priority truncation: model > tokens > cost > branch > session. Drop elements left-to-right on narrow terminals | M | 0.8 |
| 1.2 | **Live token counter** — Update status bar in real-time as `UsageUpdate` events arrive. Show running total only (no progress bar based on max_tokens) | M | 0.7, 1.1 |
| 1.3 | **Turn duration timer** — Show elapsed time for current turn. Wire up existing `showTurnDuration` config | S | 0.7 |
| 1.4 | **Git branch indicator** — Display current git branch in status bar. Use existing `parse_git_status_metadata` | S | 1.1 |
| 1.5 | **Color capability detection** — Probe terminal capabilities at startup (truecolor → 256 → 16 → monochrome). Cache result and apply appropriate theme fallback | M | - |

### Phase 2: Enhanced Streaming Output (Week 4-5)

**Goal**: Make the main response stream visually rich and responsive.

| Task | Description | Effort | Dependencies |
|---|---|---|---|
| 2.1 | **Live markdown rendering** — Implement block buffer system with boundary-first stabilization. Split on paragraph breaks, code blocks, lists, headings. Completed blocks parsed once and cached. Active block provisionally rendered after short idle timeout, revalidated on new content | XL | 0.7, 0.8 |
| 2.2 | **Thinking indicator** — Distinct animated indicator (`🧠 Reasoning...`) for extended thinking. Different spinner style from regular generation | S | 0.7 |
| 2.3 | **Remove artificial stream delay** — Remove 8ms sleep from `stream_markdown`. Immediate rendering for main response; optional delay for tool results only | S | - |
| 2.4 | **Terminal resize handling** — Listen for SIGWINCH. Store content in layout-independent form. Reflow only visible region on resize | M | 0.8 |

### Phase 3: Tool Call Visualization (Week 6-7)

**Goal**: Make tool execution legible and navigable.

| Task | Description | Effort | Dependencies |
|---|---|---|---|
| 3.1 | **Live expandable tool output** — Scrollable truncated window (default 15 lines) that updates as new lines arrive. Show `[+Expand]` hint. Live expansion with viewport during streaming | M | 0.8 |
| 3.2 | **Large output handling** — Interrupt with offer when output exceeds threshold: "Large output detected. Save to file and show summary? Y/N" | M | 3.1 |
| 3.3 | **Syntax-highlighted tool results** — Apply syntect highlighting using language detection heuristics (file extension, shebang, tool name). Fast path for known languages, fallback to plain text | M | - |
| 3.4 | **Tool call timeline** — Compact summary after multi-tool turns: `🔧 bash → ✓ | read_file → ✓ | edit_file → ✓ (3 tools, 1.2s)` | S | 0.7 |
| 3.5 | **Diff-aware edit_file display** — Show colored unified diff with word-level highlighting on edit success | M | 0.8 |
| 3.6 | **Permission prompt enhancement** — Style with box drawing, color tool name, show one-line summary of action | S | 0.8 |

### Phase 4: Enhanced Slash Commands & Navigation (Week 8-9)

**Goal**: Improve information display and add missing features.

| Task | Description | Effort | Dependencies |
|---|---|---|---|
| 4.1 | **Colored `/diff` output** — Parse git diff and render with word-level highlighting. Red/green for additions/removals, intra-line word diffs for precision | M | 0.8 |
| 4.2 | **Internal pager** — For outputs exceeding terminal height (`/status`, `/config`, `/memory`, `/diff`). Scroll with j/k/q or arrows | M | 0.8 |
| 4.3 | **`/search` command** — Search conversation history by keyword | M | - |
| 4.4 | **`/undo` command** — Undo last file edit by restoring from `originalFile` data in tool results | M | - |
| 4.5 | **Interactive session picker** — Replace text-based `/session list` with fuzzy-filterable interactive list (up/down arrows, enter to switch) | L | 0.8 |
| 4.6 | **Tab completion for tool arguments** — Extend completion for file paths, model names, session IDs | M | - |

### Phase 5: Color Themes & Configuration (Week 10)

**Goal**: User-customizable visual appearance.

| Task | Description | Effort | Dependencies |
|---|---|---|---|
| 5.1 | **Named color themes** — Add `dark` (default), `light`, `solarized`, `catppuccin` themes. Wire to existing `Config` tool's theme setting | M | 1.5 |
| 5.2 | **Configurable spinner style** — Allow choosing between braille dots, bar, moon phases, etc. | S | - |
| 5.3 | **Banner customization** — Make ASCII art banner optional/configurable via settings | S | - |

### Phase 6: Persistent Async Input System (Week 11-12)

**Goal**: Implement the persistent async input composer from the PRD.

| Task | Description | Effort | Dependencies |
|---|---|---|---|
| 6.1 | **Custom input handler** — Replace rustyline with custom line editor using crossterm event loop. Minimal but competent feature set: cursor movement, backspace, basic history, multi-line (Shift+Enter) | XL | 0.8 |
| 6.2 | **Message queue system** — FIFO queue for messages submitted during generation. Visual feedback: "queued · will send after current response" | M | 0.7 |
| 6.3 | **Queue persistence** — Append-only journal (.queue file) with atomic writes via rename. Replay on load. Handle partial writes gracefully | L | 6.2 |
| 6.4 | **Slash command overlay** — Trigger on `/`. Real-time filtering, keyboard navigation (↑↓), Enter/Tab to select, Esc to dismiss. Render over content (don't clear) | M | 0.8, 6.1 |
| 6.5 | **Two-stage interrupt** — First Ctrl+C cancels current generation, second press exits app | M | 0.8 |
| 6.6 | **Session management** — PID file for concurrent instance detection. Offer to "attach to existing session" or "start new session" | M | 6.3 |

---

## 3. Priority Recommendation

### Critical Path (Must complete first)

1. **Phase 0.1-0.8** — Foundation is critical. The 3,159-line `main.rs`, custom event loop, and ratatui adoption block everything else.
2. **Phase 6.1** — Custom input handler is blocker for async input. Must complete before queue system.

### High Impact, Moderate Effort (Quick wins)

3. **Phase 1.1-1.5** — Status bar with live tokens + capability detection. Highest-impact UX win.
4. **Phase 2.3** — Remove artificial delay. Zero-effort, immediately noticeable.
5. **Phase 3.1** — Live expandable tool output. Large bash outputs currently wreck readability.
6. **Phase 6.5** — Two-stage interrupt. Improves perceived responsiveness immediately.

### Core Features (Main functionality)

7. **Phase 2.1** — Live markdown rendering with block buffer. The streaming UX centerpiece.
8. **Phase 3.3** — Syntax-highlighted tool results. Professional polish.
9. **Phase 3.5** — Diff-aware edit display. Makes code changes immediately visible.
10. **Phase 4.1** — Colored `/diff` with word-level highlighting.
11. **Phase 6.2-6.4** — Message queue + slash command overlay. The async input composer core.

### Polish & Extensions (Nice to have)

12. **Phase 5** — Color themes (user demand-driven).
13. **Phase 4.2-4.6** — Enhanced navigation (pager, search, session picker).
14. **Phase 2.2, 2.4, 3.2, 3.4, 3.6** — Various UX improvements (thinking indicator, resize handling, timeline, permission prompts).

---

## 4. Architecture Recommendations

### Module Structure After Phase 0-1

```
crates/rusty-claude-cli/src/
├── main.rs              # Entrypoint, arg dispatch only (~100 lines)
├── args.rs              # CLI argument parsing (consolidated)
├── app.rs               # LiveCli struct, REPL loop, turn execution
├── format.rs            # All report formatting (status, cost, model, permissions, etc.)
├── session_mgr.rs       # Session CRUD: create, resume, list, switch, persist
├── init.rs              # Repo initialization (unchanged)
├── input/
│   ├── mod.rs           # Input module
│   ├── handler.rs       # Custom input handler (replaces rustyline)
│   └── history.rs       # Input history management
├── render.rs            # TerminalRenderer, Spinner (extended)
└── tui/
    ├── mod.rs           # TUI module root
    ├── event_loop.rs    # Custom event loop with tokio::select!
    ├── renderer.rs      # LayeredRenderer trait + impl
    ├── layers/
    │   ├── mod.rs       # Layer module
    │   ├── output.rs    # Output layer (scrolling)
    │   ├── input.rs     # Input layer (fixed bottom)
    │   └── overlay.rs   # Overlay layer (suggestions)
    ├── components/
    │   ├── mod.rs       # Components module
    │   ├── status_bar.rs # Persistent bottom status line
    │   ├── markdown.rs   # Block buffer markdown renderer
    │   ├── tool_panel.rs # Tool call visualization
    │   ├── diff_view.rs  # Colored diff rendering
    │   ├── pager.rs      # Internal pager
    │   ├── queue.rs      # Message queue display
    │   └── suggest.rs    # Slash command suggestions
    ├── events.rs         # RenderEvent enum, UiState struct
    └── theme.rs          # Color theme definitions and capability detection
```

### Key Design Principles (Updated)

1. **Unified layered renderer** — Single rendering system for both inline and full-screen modes. Ratatui provides layout/terminal management from Phase 0.
2. **Event-based architecture** — Stream handlers send events via mpsc channels to UI task. Clean separation of concerns.
3. **Throttled event-driven rendering** — Max 60fps with event coalescing. Smooth animations without excessive redraws.
4. **Custom event loop** — tokio::select! multiplexes stream events, input events, and render ticks.
5. **Everything testable in layers** — Unit tests for state/logic, mock terminal snapshots for rendering, PTY integration tests for real behavior.
6. **Full cross-platform** — Linux, macOS, Windows parity from day one. Use crossterm's abstraction, test on all platforms.
7. **Capability detection** — Probe terminal capabilities at startup, cache result, apply appropriate theme fallback.
8. **No feature-gating ratatui** — Adopt as core dependency from Phase 0; build custom renderer on top of its terminal backend.

---

## 5. Risk Assessment

| Risk | Mitigation | Probability | Impact |
|---|---|---|---|
| Breaking the working REPL during refactor | Phase 0 is pure restructuring with existing test coverage as safety net. Incremental migration. | Medium | High |
| Terminal compatibility issues (tmux, SSH, Windows) | Rely on crossterm's abstraction; test on all platforms from day one; capability detection with fallback. | Low | Medium |
| Performance regression with rich rendering | Profile before/after; event coalescing and throttled rendering prevent excessive redraws; keep fast path available. | Medium | Medium |
| Custom input handler UX degradation | Launch with minimal but competent feature set; incrementally regain rustyline-level affordances through shared event loop. | High | High |
| Scope creep into too many features | Phased approach with clear dependencies. Ship Phases 0-2 as foundation first. | Medium | Medium |
| Ratatui widget trait incompatibility with streaming | Build custom renderer using ratatui's terminal backend directly, bypassing widget trait. Document this pattern. | Low | Low |
| Queue file corruption on crash | Append-only journal format with atomic writes via rename. Handle partial writes on load. | Low | Medium |
| Concurrent session corruption | PID file detection; offer to attach or start new session. Never allow simultaneous writes. | Low | High |
| Block buffer markdown parsing edge cases | Boundary-first stabilization with timeout fallback. Provisional rendering for active block. | Medium | Low |
| Cross-platform testing burden | Layered testing strategy: unit tests (fast), mock snapshots (deterministic), minimal PTY integration tests (slow but comprehensive). | Medium | Medium |
| Message queue persistence complexity | Start with in-memory queue; add persistence in Phase 6.3 after queue system is stable. | Low | Medium |

### Critical Success Factors

1. **Phase 0 completion** — Cannot proceed without clean module structure and event loop foundation
2. **Custom input handler quality** — UX must feel natural; regression from rustyline will be immediately noticeable
3. **Event loop correctness** — Must handle all edge cases (stream interruption, resize, rapid input) without freezing or dropping events
4. **Markdown rendering performance** — Block buffer system must be fast enough for real-time streaming
5. **Cross-platform testing** — Must catch Windows/console edge cases before they reach users

### Contingency Plans

| If... | Then... |
|---|---|
| Custom input handler proves too complex | Keep rustyline for input, use crossterm only for output layers. Accept async input limitation. |
| Block buffer markdown is too slow | Fall back to plain text streaming, apply markdown rendering only on turn completion. |
| Ratatui adoption creates platform issues | Drop ratatui, use crossterm directly. Custom layout code but fewer dependencies. |
| Queue persistence causes data loss | Document as "best effort" feature. Add explicit warning on startup if queue exists from crash. |
| Scope creep threatens timeline | Cut Phase 4-6 features. Focus on Phases 0-3 as "MVP" layered renderer. |

---

*Generated: 2026-03-31 | Updated: 2026-04-24 (Technical Interview Integration)*
*Workspace: `rust/` | Branch: `dev/rust`*
