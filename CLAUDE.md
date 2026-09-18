# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Test Commands

```bash
# Build the entire workspace in release mode
cargo build --release

# Build specific crate
cargo build -p rusty-claude-cli --release

# Run the CLI (after building)
./target/release/oog

# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run mock parity harness (comprehensive end-to-end test)
cargo test -p rusty-claude-cli --test mock_parity_harness -- --nocapture
# Or use the script: ./scripts/run_mock_parity_harness.sh

# Run linter (all warnings are errors)
cargo clippy --all-targets --all-features -- -D warnings

# Development run with REPL
cargo run -p rusty-claude-cli

# One-shot prompt
cargo run -p rusty-claude-cli -- prompt "explain this codebase"

# With specific model
cargo run -p rusty-claude-cli -- --model sonnet prompt "fix the bug"
```

## Architecture Overview

This is a Rust workspace implementing a high-performance CLI AI coding assistant called **Oog Code**.

### Workspace Structure

```
crates/
├── rusty-claude-cli/     # Main CLI binary (REPL, one-shot prompts, args)
├── runtime/              # Core execution engine (sessions, config, permissions, MCP)
├── api/                  # HTTP client, SSE streaming, OAuth, API types
├── tools/                # Built-in tool implementations (bash, read, write, edit, grep, glob)
├── commands/             # Slash command registry and help generation
├── compat-harness/       # Extracts tool/prompt manifests from upstream TypeScript
├── mock-anthropic-service/  # Deterministic Anthropic-compatible mock for testing
├── plugins/              # Plugin system foundation
└── telemetry/            # Basic telemetry utilities
```

### Crate Dependencies & Interactions

```
rusty-claude-cli (main entry point)
├── api (HTTP client with streaming)
├── runtime (core logic)
│   ├── tools (tool execution)
│   ├── plugins (plugin system)
│   └── telemetry
├── commands (slash commands)
└── compat-harness (manifest extraction)
```

### Key Architectural Patterns

**Monolithic Main Structure**: The `rusty-claude-cli/src/main.rs` is large (~3,000 lines) and orchestrates:
- REPL (via rustyline) for interactive sessions
- One-shot prompt execution
- Tool rendering and markdown display
- CLI argument parsing (via clap)

**Streaming-First Design**: Full Server-Sent Events (SSE) streaming support with real-time markdown rendering using pulldown-cmark and syntect for syntax highlighting.

**Permission System**: Three-tier permission enforcement:
- `ReadOnly` - No filesystem writes
- `WorkspaceWrite` - Writes within workspace only
- `DangerFullAccess` - Unrestricted access

**Session Management**: Persistent conversation sessions with:
- JSONL-based session storage
- Session resume via `/session` command
- Conversation compaction for token efficiency

**MCP Integration**: Model Context Protocol server lifecycle management for external tools.

### Important Technical Details

**Entry Point**: Binary name is `oog`, defined in `crates/rusty-claude-cli/Cargo.toml`.

**Authentication**: Supports both API key (`TROGLODYTIC_API_KEY`, with legacy `MINSROPIC_API_KEY`) and OAuth PKCE flow (`oog login`).

**Model Aliases**: Short names resolve to latest versions:
- `opus` → `claude-opus-4-6`
- `sonnet` → `claude-sonnet-4-6`
- `haiku` → `claude-haiku-4-5-20251213`

**Cost Tracking**: Real-time token usage and cost display via `/status` and `/cost` commands.

**Git Integration**: Built-in git diff support and version control awareness.

**Slash Commands**: Rich REPL commands with tab completion for commands, model aliases, permission modes, and session IDs.

**Testing Infrastructure**: Mock parity harness provides deterministic end-to-end testing. Run with `./scripts/run_mock_parity_harness.sh`.

### Configuration

Environment variables:
- `TROGLODYTIC_API_KEY` - API authentication (legacy `MINSROPIC_API_KEY` is also accepted)
- `TROGLODYTIC_BASE_URL` - Optional proxy/base URL (legacy `MINSROPIC_BASE_URL` is also accepted)

Config file hierarchy: `.claude.json` (not present by default but supported).

### Linting Standards

- **Unsafe code**: Forbidden (`unsafe_code = "forbid"`)
- **Clippy**: All warnings are errors (`-D warnings`)
- **Pedantic lints**: Enabled with selective allows

### Key Dependencies

- **reqwest** - HTTP client with streaming
- **tokio** - Async runtime
- **rustyline** - REPL functionality
- **pulldown-cmark** - Markdown parsing
- **syntect** - Syntax highlighting
- **crossterm** - Terminal handling
