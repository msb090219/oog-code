# Oog Code: Product Direction

## Decision

Oog Code is a deliberately small, token-efficient coding CLI: **small brain, sharp club**.

It is not a conventional coding platform with caveman wording pasted over it. It should solve normal coding tasks with a tiny visible interface, a small default tool set, and compact, technically exact responses.

## Product character

- Oog speaks briefly and plainly: short facts, exact evidence, no throat-clearing.
- Oog is warm and playful, but never sacrifices clarity, code, paths, errors, security warnings, or confirmation language for the joke.
- Coding work is natural language: `fix auth expiry`, `find callers`, `run tests`.
- Slash commands are only for controlling the CLI itself.

Do not use Caveman branding in product prompts or UI. Caveman is inspiration for compression discipline, not Oog Code's identity.

## Small CLI surface

The default REPL command set should be one screen:

| Command | Purpose |
|---|---|
| `/see` | Show session status, diff, and active context. |
| `/small` | Show or enable lean-context mode and token usage. |
| `/smash` | Compact conversation history. |
| `/clear` | Start a fresh session; require confirmation. |
| `/cost` | Show token and cost breakdown. |
| `/help` | Show this command list. |
| `/quit` | Exit. |

Everything else is normal language. Existing specialist slash commands should be hidden first, then removed when compatibility no longer needs them. Do not replace one long command catalogue with themed synonyms.

## Tool design

Keep model-facing tool names conventional and reliable:

- `read_file`
- `grep_search`
- `edit_file`
- `write_file`
- `bash`
- `ToolSearch`

Use caveman-flavoured terminal labels, not renamed protocol tools:

- `Oog look: src/auth.rs`
- `Oog sniff: token expiry`
- `Oog carve: src/auth.rs`
- `Oog bonk: cargo test`

Specialist tools must be genuinely deferred. `ToolSearch` discovers them; only selected tools are sent in later model requests. A tool that is merely hidden in the UI but still included in the schema saves no tokens.

## Token rules

1. Measure provider-reported input, output, cache-read, and cache-write tokens per turn.
2. Attribute input token cost to system prompt, tool schemas, history, and tool results.
3. Keep only the six core tool schemas in a normal request.
4. Store full tool output locally, but send a compact result that preserves errors, stack traces, changed files, and decisive first/last lines. Provide a retrieval handle for the original.
5. Compact based on live estimated context, not only cumulative lifetime usage.
6. Never compress user prompts, source code, commands, exact errors, security warnings, or destructive-action confirmations.

## First implementation sequence

1. Make tool deferral real and make the six-tool profile the default.
2. Add `/small` token attribution so every reduction is visible and testable.
3. Replace the current `Caveman Code voice` prompt section with Oog-owned speech rules.
4. Reduce the slash-command registry to the small control surface.
5. Add loss-aware tool-result compression only after the ledger identifies noisy result types.

## Success criteria

- A normal request sends six core tools, not the entire built-in catalogue.
- `/help` fits on one terminal screen and exposes only CLI control commands.
- Oog terminal labels feel caveman-inspired while model tool contracts remain stable.
- Lean mode demonstrably lowers provider-reported input tokens on the mock parity scenarios without changing expected task outcomes.

## Explicit non-goals

- Do not copy Caveman branding, prompts, or source code wholesale.
- Do not embed Caveman's BSL-licensed compression engine or proxy.
- Do not add specialist commands or tool schemas without measured demand.
