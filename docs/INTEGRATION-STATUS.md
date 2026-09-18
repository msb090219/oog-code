# TUI integration — retired

**Decision:** Oog Code will keep its original inline terminal interface.
It will not use ratatui, an alternate screen, a layered renderer, or an async
input composer.

The current CLI uses `rustyline` for input and writes streaming ANSI/Markdown
output to normal terminal scrollback. That is the supported product path.

The earlier TUI documents are historical experiments, not active integration
work. The unintegrated `new_renderer/` demo and its `/renderer` commands were
removed. Do not create `src/tui/` or add a ratatui dependency as follow-up
work.

For active work, see the [CLI roadmap](planning/CLI-ROADMAP.md).
