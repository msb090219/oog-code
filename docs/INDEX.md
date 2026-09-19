# Oog Code documentation

Oog Code is a terminal-first AI coding assistant. Its product interface is
the existing inline REPL and one-shot CLI: normal terminal scrollback,
`rustyline` input, and ANSI/Markdown output. It is not a full-screen TUI.

## Start here

- [Oog Code direction](OOG_CODE_DIRECTION.md) — the product brief and boundaries.
- [Oog Code implementation plan](planning/OOG_CODE_IMPLEMENTATION_PLAN.md) — the active delivery plan.
- [CLI roadmap](planning/CLI-ROADMAP.md) — older terminal-first baseline and reliability notes.
- [Architecture](architecture/ARCHITECTURE.md) — what is actually wired today.
- [Features](reference/FEATURES.md) — implemented tool and command surface.
- [Parity](reference/PARITY.md) — upstream-compatibility work and known gaps.
- [Testing guide](reference/TESTING_GUIDE.md) — validation commands.

## Retired TUI work

The previous ratatui/layered-renderer proposal was intentionally abandoned.
Do not implement or revive it unless the product direction changes explicitly.

- [Retirement record](INTEGRATION-STATUS.md)
- [Historical TUI proposal](planning/TUI-ENHANCEMENT-PLAN.md)
- [Historical async-input PRD](planning/USER_INPUT_PRD.md)

Those documents are retained for history only; they are not roadmap items and
do not describe the current source tree.
