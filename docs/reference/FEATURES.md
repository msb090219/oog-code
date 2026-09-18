#  Oog Code - Feature & Capability Summary

**Last Updated:** 2026-04-05

## Overview

Oog Code is a personal AI coding assistant built in Rust, with custom features and capabilities tailored for personal development workflows.

---

## Core Capabilities

### 1. AI-Powered Code Assistance
- Interactive REPL with `rustyline` (tab completion, history)
- One-shot prompt execution via `minseo prompt "command"`
- Streaming responses from Anthropic-compatible APIs
- Extended thinking support (thinking blocks)
- Model aliases: `opus`, `sonnet`, `haiku`

### 2. Authentication
- API key authentication
- OAuth login/logout with PKCE flow
- Secure credential storage
- Proxy support for custom endpoints (e.g., Zhipu AI GLM)

### 3. Permission System
Three permission modes with enforcement across all tools:
- `read-only` - No modifications allowed
- `workspace-write` - Write only within workspace
- `danger-full-access` - Full system access
- Permission prompts for sensitive operations
- Sandbox support for Linux environments

---

## Tool System (40/40 Tools)

### File Operations (5 tools)
| Tool | Capability | Status |
|------|-----------|--------|
| `read_file` | Read files with offset/limit support | Complete |
| `write_file` | Create/overwrite files | Complete |
| `edit_file` | String replacement editing | Complete* |
| `glob_search` | Pattern-based file search | Complete |
| `grep_search` | Content search via ripgrep | Complete |

*Missing: `replace_all` functionality

### Shell Operations (2 tools)
| Tool | Capability | Status |
|------|-----------|--------|
| `bash` | Subprocess execution with timeout, background, sandbox | Complete (9/9 validation submodules) |
| `PowerShell` | Windows PowerShell execution | Complete |

**Bash Validation Submodules (all complete):**
- sedValidation
- pathValidation
- readOnlyValidation
- destructiveCommandWarning
- commandSemantics
- bashPermissions
- bashSecurity
- modeValidation
- shouldUseSandbox

### Web Tools (2 tools)
| Tool | Capability | Status |
|------|-----------|--------|
| `WebFetch` | URL content fetching | Complete |
| `WebSearch` | Search query execution | Complete |

### Agent Orchestration (1 tool)
| Tool | Capability | Status |
|------|-----------|--------|
| `Agent` | Sub-agent delegation | Complete |

### Task Management (6 tools)
| Tool | Capability | Status |
|------|-----------|--------|
| `TaskCreate` | Create in-memory tasks | Complete |
| `TaskGet` | Task lookup + metadata | Complete |
| `TaskList` | List all tasks | Complete |
| `TaskStop` | Stop running tasks | Complete |
| `TaskUpdate` | Update task messages | Complete |
| `TaskOutput` | Retrieve task output | Complete |

### Team & Cron (4 tools)
| Tool | Capability | Status |
|------|-----------|--------|
| `TeamCreate` | Create teams with task assignment | Complete |
| `TeamDelete` | Delete teams | Complete |
| `CronCreate` | Create cron entries | Complete |
| `CronDelete` | Delete cron entries | Complete |
| `CronList` | List cron entries | Complete |

### MCP Integration (3 tools)
| Tool | Capability | Status |
|------|-----------|--------|
| `MCP` | Stateful MCP tool invocation | Complete |
| `ListMcpResources` | List connected server resources | Complete |
| `ReadMcpResource` | Read MCP resources | Complete |

### LSP Integration (1 tool)
| Tool | Capability | Status |
|------|-----------|--------|
| `LSP` | Diagnostics, hover, definition, references, completion, symbols, formatting | Complete |

### Additional Tools (17 tools)
| Tool | Capability | Status |
|------|-----------|--------|
| `TodoWrite` | Todo/note persistence | Complete |
| `NotebookEdit` | Jupyter notebook cell editing | Complete |
| `Skill` | Skill discovery/install | Complete |
| `ToolSearch` | Tool discovery | Complete |
| `Sleep` | Delay execution | Complete |
| `SendUserMessage/Brief` | User-facing messages | Complete |
| `Config` | Config inspection | Complete |
| `EnterPlanMode` | Toggle plan mode | Complete |
| `ExitPlanMode` | Restore from plan mode | Complete |
| `StructuredOutput` | Passthrough JSON | Complete |
| `REPL` | Subprocess code execution | Complete |

### Stub Tools (4 tools)
| Tool | Status | Notes |
|------|--------|-------|
| `AskUserQuestion` | Stub only | Needs live user I/O integration |
| `McpAuth` | Stub only | Needs full auth UX |
| `RemoteTrigger` | Stub only | Needs HTTP client |
| `TestingPermission` | Stub only | Test-only, low priority |

---

## Slash Commands (67/141)

### Implemented Commands (27)
- `/help` - Show help
- `/status` - Show session status (model, tokens, cost)
- `/cost` - Show cost breakdown
- `/compact` - Compact conversation history
- `/clear` - Clear conversation
- `/model [name]` - Show or switch model
- `/permissions` - Show or switch permission mode
- `/config [section]` - Show config (env, hooks, model)
- `/memory` - Show CLAUDE.md contents
- `/diff` - Show git diff
- `/export [path]` - Export conversation
- `/session [id]` - Resume previous session
- `/version` - Show version

### New Specs (40)
- Parse + stub handler ("not yet implemented")

### Missing (~74)
- Internal modules/dialogs/steps (not user-facing `/commands`)

---

## Runtime Features

### Session Management
- Session persistence + resume
- Conversation compaction

### Usage Tracking
- Cost tracking + usage display
- Usage estimation

### Git Integration
- Git status
- Git diff
- Git operations (commit, push, pull)

### Output Formatting
- Markdown terminal rendering (ANSI)

### Streaming
- SSE (Server-Sent Events) streaming
- Incremental response parsing

---

## Configuration

### Config File Hierarchy
- `.claude.json` support
- Config loader hierarchy

### MCP Configuration
- MCP server lifecycle (connect, list tools, call tool, disconnect)
- Multiple server types (stdio, SDK, WebSocket, remote, managed proxy)

### Hooks
- PreToolUse/PostToolUse hooks (config only)

---

## Architecture

### Workspace Structure (7 crates)
```
crates/
├── api/                    # API client + SSE streaming
├── commands/               # Shared slash-command registry
├── compat-harness/         # TS manifest extraction harness
├── mock-anthropic-service/ # Deterministic local mock for testing
├── runtime/                # Session, config, permissions, MCP, prompts
├── rusty-claude-cli/       # Main CLI binary (`oog`)
└── tools/                  # Built-in tool implementations
```

### Safety Features
- `unsafe_code = "forbid"` - No unsafe Rust allowed
- Comprehensive linting (clippy pedantic)
- Permission enforcement across all tools
- Sandbox support for Linux
- Path traversal prevention
- Binary file detection
- Size limits on read/write

---

## Testing & Parity

### Mock Parity Harness
- Deterministic Anthropic-compatible mock service
- Clean-environment CLI harness
- 10 scripted test scenarios
- Behavioral diff/checklist runner

### Test Scenarios
1. `streaming_text`
2. `read_file_roundtrip`
3. `grep_chunk_assembly`
4. `write_file_allowed`
5. `write_file_denied`
6. `multi_tool_turn_roundtrip`
7. `bash_stdout_roundtrip`
8. `bash_permission_prompt_approved`
9. `bash_permission_prompt_denied`
10. `plugin_tool_roundtrip`

---

## Project Stats

- **~20K lines** of Rust code
- **40/40 tools** with real implementations
- **67/141 slash commands** implemented
- **7 crates** in workspace
- **Binary name:** `oog`
- **Default model:** `claude-opus-4-6`
- **Default permissions:** `danger-full-access`

---

## CLI Usage

### Build
```bash
cargo build --release
```

### Interactive REPL
```bash
./target/release/oog
```

### One-shot Prompt
```bash
./target/release/oog prompt "explain this codebase"
```

### With Model Selection
```bash
./target/release/oog --model sonnet prompt "fix the bug"
```

### CLI Flags
- `--model MODEL` - Set model (alias or full name)
- `--dangerously-skip-permissions` - Skip permission checks
- `--permission-mode MODE` - Set permission mode
- `--allowedTools TOOLS` - Restrict enabled tools
- `--output-format FORMAT` - Output format (text/json)
- `--version, -V` - Print version info

---

## Roadmap

### Planned Features
- Plugin system (install/enable/disable/uninstall)
- Skills registry
- Full hooks execution system
- Custom features unique to Oog Code

### Ongoing Improvements
- Output truncation for large content
- Session compaction refinement
- Token counting accuracy

---

*This document summarizes the current state of Oog Code as of April 2026.*
