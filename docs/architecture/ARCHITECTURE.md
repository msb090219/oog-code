# Architecture overview

Oog Code is a Rust workspace for a terminal-first AI coding assistant.

```
oog CLI
  -> runtime (conversation loop, sessions, permissions, config, MCP)
  -> api     (providers, authentication, SSE streaming)
  -> tools   (built-in tool specifications and execution)
```

`crates/rusty-claude-cli/src/main.rs` owns the interactive REPL, one-shot
commands, terminal rendering, and command dispatch. Input is provided by
`rustyline`; assistant output is streamed to normal terminal scrollback using
`render.rs`. This deliberate architecture does not include an alternate-screen
TUI, ratatui, an event-loop UI layer, or persistent async input.

Supporting crates:

- `commands` supplies shared slash-command definitions and help.
- `plugins` supplies plugin discovery and lifecycle foundations.
- `mock-anthropic-service` and `compat-harness` support deterministic parity
  testing.
- `telemetry` contains telemetry utilities.

The unintegrated `new_renderer/` demo and its `/renderer` commands were
removed; `render.rs` is the only live rendering path. The active work sequence
is in the [CLI roadmap](../planning/CLI-ROADMAP.md).
