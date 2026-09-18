# Oog Code — Operating On Guesses

Oog Code is Troglodytic's high-performance Rust CLI AI coding assistant for vibe coding: fast, safety-aware, and built around native tool execution.

## Documentation

The detail lives in [`docs/`](docs/):

- **[Documentation Index](docs/INDEX.md)** - Master index for all documentation
- **[CLI Roadmap](docs/planning/CLI-ROADMAP.md)** - Active terminal-first product plan
- **[Architecture Overview](docs/architecture/ARCHITECTURE.md)** - System architecture and component interactions
- **[Retired TUI Work](docs/INTEGRATION-STATUS.md)** - Historical record; not an active roadmap

### Quick Links
- **[Build Commands](CLAUDE.md)** - How to build, test, and run
- **[Features](docs/reference/FEATURES.md)** - Complete feature list
- **[Design System](docs/architecture/DESIGN_SYSTEM.md)** - Visual design guidelines

## Quick Start

```bash
# Build
cargo build --release

# Run interactive REPL
./target/release/oog

# One-shot prompt
./target/release/oog prompt "explain this codebase"

# With specific model
./target/release/oog --model sonnet prompt "fix the bug in main.rs"
```

## Configuration

Set your API credentials:

```bash
export TROGLODYTIC_API_KEY="your-api-key"
# Or use a proxy (e.g., Zhipu AI GLM)
export TROGLODYTIC_BASE_URL="https://open.bigmodel.cn/api/paas/v4"
# Legacy MINSROPIC_API_KEY and MINSROPIC_BASE_URL names remain supported.
```

Or authenticate via OAuth:

```bash
oog login
```

Configuration is loaded from `~/.minseo/settings.json`, `.minseo.json`, and
project `.minseo/settings.json` or `.minseo/settings.local.json`; legacy
`.claw` locations remain supported.

## Mock parity harness

The workspace now includes a deterministic Anthropic-compatible mock service and a clean-environment CLI harness for end-to-end parity checks.

```bash
# Run the scripted clean-environment harness
./scripts/run_mock_parity_harness.sh

# Or start the mock service manually for ad hoc CLI runs
cargo run -p mock-anthropic-service -- --bind 127.0.0.1:0
```

Harness coverage:

- `streaming_text`
- `read_file_roundtrip`
- `grep_chunk_assembly`
- `write_file_allowed`
- `write_file_denied`
- `multi_tool_turn_roundtrip`
- `bash_stdout_roundtrip`
- `bash_permission_prompt_approved`
- `bash_permission_prompt_denied`
- `plugin_tool_roundtrip`

Primary artifacts:

- `crates/mock-anthropic-service/` — reusable mock Anthropic-compatible service
- `crates/rusty-claude-cli/tests/mock_parity_harness.rs` — clean-env CLI harness
- `scripts/run_mock_parity_harness.sh` — reproducible wrapper
- `scripts/run_mock_parity_diff.py` — scenario checklist + PARITY mapping runner
- `mock_parity_scenarios.json` — scenario-to-PARITY manifest

## Features

| Feature | Status |
|---------|--------|
| Anthropic API + streaming | ✅ |
| OAuth login/logout | ✅ |
| Interactive REPL (rustyline) | ✅ |
| Tool system (bash, read, write, edit, grep, glob) | ✅ |
| Web tools (search, fetch) | ✅ |
| Sub-agent orchestration | ✅ |
| Todo tracking | ✅ |
| Notebook editing | ✅ |
| CLAUDE.md / project memory | ✅ |
| Config file hierarchy (.claude.json) | ✅ |
| Permission system | ✅ |
| MCP server lifecycle | ✅ |
| Session persistence + resume | ✅ |
| Extended thinking (thinking blocks) | ✅ |
| Cost tracking + usage display | ✅ |
| Git integration | ✅ |
| Markdown terminal rendering (ANSI) | ✅ |
| Model aliases (opus/sonnet/haiku) | ✅ |
| Slash commands (/status, /compact, /clear, etc.) | ✅ |
| Hooks (PreToolUse/PostToolUse) | 🔧 Config only |
| Plugin management | ✅ Install, enable, disable, update, and uninstall local plugins |
| Skills registry | ✅ List and install available skills |

## Model Aliases

Short names resolve to the latest model versions:

| Alias | Resolves To |
|-------|------------|
| `opus` | `claude-opus-4-6` |
| `sonnet` | `claude-sonnet-4-6` |
| `haiku` | `claude-haiku-4-5-20251213` |

## CLI Flags

```
oog [OPTIONS] [COMMAND]

Options:
  --model MODEL                    Set the model (alias or full name)
  --dangerously-skip-permissions   Skip all permission checks
  --permission-mode MODE           Set read-only, workspace-write, or danger-full-access
  --allowedTools TOOLS             Restrict enabled tools
  --output-format FORMAT           Output format (text or json)
  --version, -V                    Print version info

Commands:
  prompt <text>      One-shot prompt (non-interactive)
  login              Authenticate via OAuth
  logout             Clear stored credentials
  init               Initialize project config
  status             Show a local workspace status snapshot
  sandbox            Show the sandbox isolation snapshot
  agents             Inspect configured agents
  mcp                Inspect configured MCP servers
  skills             List or install skills
```

## Slash Commands (REPL)

Tab completion now expands not just slash command names, but also common workflow arguments like model aliases, permission modes, and recent session IDs.

| Command | Description |
|---------|-------------|
| `/help` | Show help |
| `/status` | Show session status (model, tokens, cost) |
| `/cost` | Show cost breakdown |
| `/compact` | Compact conversation history |
| `/clear` | Clear conversation |
| `/model [name]` | Show or switch model |
| `/permissions` | Show or switch permission mode |
| `/config [section]` | Show config (env, hooks, model) |
| `/memory` | Show CLAUDE.md contents |
| `/diff` | Show git diff |
| `/export [path]` | Export conversation |
| `/session [id]` | Resume a previous session |
| `/version` | Show version |
| `/sandbox` | Show sandbox isolation status |
| `/session` | List, title, switch, fork, or delete local sessions |
| `/plugin` | Manage local plugins |
| `/agents`, `/skills`, `/mcp` | Inspect available agents, skills, and MCP servers |
| `/commit`, `/pr`, `/issue` | Create a commit or draft GitHub work from the conversation |
| `/bughunter`, `/ultraplan` | Run code analysis or deep planning |

## Workspace Layout

```
.
├── Cargo.toml              # Workspace root
├── Cargo.lock
└── crates/
    ├── api/                # Anthropic API client + SSE streaming
    ├── commands/           # Shared slash-command registry
    ├── compat-harness/     # TS manifest extraction harness
    ├── mock-anthropic-service/ # Deterministic local Anthropic-compatible mock
    ├── plugins/            # Plugin discovery and lifecycle
    ├── runtime/            # Session, config, permissions, MCP, prompts
    ├── rusty-claude-cli/   # Main CLI binary (`oog`)
    ├── telemetry/          # Telemetry utilities
    └── tools/              # Built-in tool implementations
```

### Crate Responsibilities

- **api** — HTTP client, SSE stream parser, request/response types, auth (API key + OAuth bearer)
- **commands** — Slash command definitions and help text generation
- **compat-harness** — Extracts tool/prompt manifests from upstream TS source
- **mock-anthropic-service** — Deterministic `/v1/messages` mock for CLI parity tests and local harness runs
- **runtime** — `ConversationRuntime` agentic loop, `ConfigLoader` hierarchy, `Session` persistence, permission policy, MCP client, system prompt assembly, usage tracking
- **rusty-claude-cli** — REPL, one-shot prompt, streaming display, tool call rendering, CLI argument parsing
- **plugins** — local plugin discovery, installation, enablement, and tool registration
- **telemetry** — telemetry utilities
- **tools** — Tool specs + execution: Bash, ReadFile, WriteFile, EditFile, GlobSearch, GrepSearch, WebSearch, WebFetch, Agent, TodoWrite, NotebookEdit, Skill, ToolSearch, REPL runtimes

## Stats

- **~20K lines** of Rust
- **9 crates** in workspace
- **Binary name:** `oog`
- **Default model:** `claude-opus-4-6`
- **Default permissions:** `danger-full-access`

## License

See repository root.
