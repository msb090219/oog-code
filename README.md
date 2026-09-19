# Oog Code — Operating On Guesses

Oog Code is a terminal-first Rust coding assistant. It keeps the normal path
small: inspect, search, edit, write, run commands, then discover specialist
tools only when they are needed.

## Quick start

```bash
cargo build --release
./target/release/oog
./target/release/oog prompt "explain this codebase"
```

Set `TROGLODYTIC_API_KEY` (or the legacy `MINSROPIC_API_KEY`) before sending a
prompt. `oog login` starts the OAuth flow.

## What works

- An inline, multiline terminal REPL with streamed Markdown and syntax-aware
  code output.
- Persistent local chats: auto-titles, resume, switch, rename, fork, delete,
  and transcript export.
- Six default tools: read, grep, edit, write, shell, and `ToolSearch`.
  Specialist tools are exposed after discovery instead of on every request.
- Read-only, workspace-write, and full-access permission modes.
- Token and context reporting, provider cost reporting, and local session
  compaction.
- Compact tool activity and end-of-turn results. Oversized shell output is
  retained beside the session and summarized in model context.
- MCP and local plugin foundations.

## REPL controls

| Command | Purpose |
| --- | --- |
| `/see` | Session, workspace, and active context (`/status` also works) |
| `/smash` | Compact chat history (`/compact` also works) |
| `/small` | Provider token usage plus local next-request estimates |
| `/session` | Open and manage saved chats |
| `/clear --confirm` | Start a fresh chat |
| `/cost` | Provider token usage and estimated cost |
| `/model`, `/permissions` | Advanced model and permission controls |
| `/help`, `/quit` | Help and exit |

## Configuration

Oog reads `~/.minseo/settings.json`, `.minseo.json`, and project
`.minseo/settings.json` or `.minseo/settings.local.json`. Legacy `.claw`
locations remain supported.

## Verify

```bash
cargo test -p oog-code --bin oog
```

## Documentation

- [Oog implementation plan](docs/planning/OOG_CODE_IMPLEMENTATION_PLAN.md)
- [Product direction](docs/OOG_CODE_DIRECTION.md)
- [Architecture](docs/architecture/ARCHITECTURE.md)
- [Testing guide](docs/reference/TESTING_GUIDE.md)

## Workspace

The workspace contains the CLI plus API, runtime, tool, command, plugin,
telemetry, compatibility-harness, and mock-service crates.
