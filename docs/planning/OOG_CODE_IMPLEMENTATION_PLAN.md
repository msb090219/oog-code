# Oog Code implementation plan

> **Status:** Active plan  
> **Product brief:** [`../OOG_CODE_DIRECTION.md`](../OOG_CODE_DIRECTION.md)  
> **Updated:** 2026-09-19

## Purpose

Build Oog Code into a small, token-efficient terminal coding CLI: an advanced
coding agent with a compact, primal presentation. Oog should be funny without
being vague, and capable without carrying a large visible command catalogue or
an always-present tool catalogue.

This document is the working implementation order. It replaces the older
feature-expansion priority in `CLI-ROADMAP.md` where they conflict. The old
roadmap remains useful as historical context and for baseline reliability work.

## Product rules

- Oog talks briefly and plainly. Code, paths, commands, exact errors, security
  warnings, and confirmations are never compressed or made jokey.
- Natural language is for coding work. Slash commands control the CLI itself.
- Model-facing tool names stay conventional. Only terminal labels are
  Oog-flavoured.
- A normal request starts with six tools only: `read_file`, `grep_search`,
  `edit_file`, `write_file`, `bash`, and `ToolSearch`.
- Specialist tools are sent only after `ToolSearch` discovers them.
- Do not revive the old full-screen TUI, copy Caveman source or branding, or
  embed its BSL-licensed engine/proxy.

## Current baseline

### Working now

- Inline terminal conversation with a custom multiline composer and streamed
  Markdown/code output.
- Persistent local sessions: create, auto-title, picker, resume, switch,
  rename, delete, and transcript replay.
- Permissions, core file/search/edit/bash tools, MCP/plugin foundations, and
  tool streaming.
- Compact user-message blocks, turn dividers, syntax-aware Markdown rendering,
  and a status row beneath the composer.
- Six-tool default profile and ToolSearch-driven deferred-tool exposure.
- Provider token/cost collection through existing status and cost reports.
- `/small` token attribution and Oog-owned concise response rules.
- Large `bash` output is stored beside its session and replaced in provider context with a compact retrieval handle.

### Known gaps

- Tool activity is functional but not a cohesive Oog activity stream.
- There is no final per-turn work summary.
- Only `bash` logs use loss-aware result compaction; other noisy tool payloads
  have not yet been measured or compacted.
- Token data is collected but not attributed by source in `/small`.
- Help/command parsing still inherits a large compatibility registry.
- Several docs still call the older CLI roadmap active.
- The mock parity harness has no runnable scenario on this Windows setup; it is
  not a sufficient release check yet.

## Delivery order

### Milestone 1 — Oog turn activity and final result

**Goal:** every coding turn visibly explains what Oog is doing, then ends with
a small factual result.

#### Build

- Render compact lifecycle rows for every tool call:
  - `Oog look` for reads/files
  - `Oog sniff` for search/discovery
  - `Oog carve` for writes/edits
  - `Oog bonk` for shell commands/tests
- Keep one start row and one result row per action. Do not print raw tool JSON
  unless it is required diagnostic output.
- Add a final turn summary after a successful assistant response:
  - changed file paths, when edits/writes occurred
  - command/test outcome, when bash was used
  - elapsed duration
  - provider token usage and estimated cost
- Show one concise error outcome on failures while retaining the exact error.

#### Acceptance checks

- A read/search/edit/test turn produces recognisable Oog activity rows.
- A completed edit/test turn names changed files, test result, elapsed time,
  and usage/cost.
- Error text remains intact.
- Unit tests cover tool-name-to-label mapping and summary extraction from a
  `TurnSummary`.

### Milestone 2 — Small, coherent command surface

**Goal:** `/help` fits on one screen and lists only controls a normal Oog user
needs.

#### Build

Expose these controls:

```text
/see      Session, workspace diff, and active context
/small    Token/context ledger
/smash    Compact conversation history
/session  Pick, resume, rename, or delete local chats
/clear    Start a fresh chat; requires confirmation
/cost     Provider token usage and cost
/help     This compact guide
/quit     Exit
```

Keep `/model` and `/permissions` as supported advanced controls. Keep
`/status`, `/compact`, and `/exit` as compatibility aliases during migration;
they should not dominate help or completions.

#### Acceptance checks

- `/help` is one screen in an 80-column terminal.
- Completion offers the visible set plus advanced controls, not the inherited
  catalogue.
- Existing saved workflows using compatibility aliases still work.
- Parsing, help, and completion tests use the same command list.

### Milestone 3 — Welcome and conversation polish

**Goal:** opening Oog and reading a conversation feel calm, intentional, and
not dashboard-like.

#### Build

- Reduce the welcome screen to a short greeting, current project, model or
  permission context, and one useful recent-session hint.
- Preserve muted three-row user blocks and direct assistant responses.
- Apply consistent blank space around dividers, tool activity, summaries, and
  the composer.
- Keep the composer visible while Oog works; it settles at the terminal bottom
  only after transcript content fills the available space.

#### Acceptance checks

- Screenshot/manual checks in Windows Terminal and PowerShell show no doubled
  input, stacked redraws, or hidden composer during streaming.
- A new session opens without a metadata wall.

### Milestone 4 — `/small` token ledger

**Goal:** make Oog's token discipline visible and testable.

#### Build

- Show provider-reported input, output, cache-read, and cache-write tokens for
  the latest turn and session total.
- Estimate request input portions separately for system prompt, active tool
  schemas, conversation history, and tool results.
- Show active tool count and any deferred tools activated through ToolSearch.
- Label estimates as estimates; never present them as provider billing data.

#### Acceptance checks

- The report distinguishes provider values from local estimates.
- A normal fresh session reports six active tools.
- A ToolSearch scenario reports the newly activated specialist tool.
- Fixed fixtures cover the attribution calculation.

### Milestone 5 — Oog-owned speech rules

**Goal:** concise behaviour comes from Oog's own product prompt, not borrowed
Caveman wording.

#### Build

- Replace the current Caveman-oriented prompt section with short Oog rules.
- Require direct answers, evidence-first coding reports, and compact tool
  narration.
- Explicitly protect source code, exact errors, commands, security warnings,
  and confirmations from compression.

#### Acceptance checks

- Prompt snapshot tests contain Oog wording and no Caveman product branding.
- Safety and destructive-action fixtures retain full clarity.

### Milestone 6 — Loss-aware tool-result handling

**Goal:** reduce large provider inputs without losing recoverability.

#### Build

- Save full oversized tool results locally per session.
- Send a compact representation that preserves errors, stack traces, changed
  files, and decisive first/last lines.
- Return a local retrieval handle that lets later work inspect the original.
- Start with plain command/log output; add payload-specific compression only
  after `/small` identifies a repeated high-cost type.

#### Acceptance checks

- The complete original is recoverable byte-for-byte.
- Compact results retain a failure's error and stack-tail evidence.
- Small tool results are not changed.
- Tests cover logs, command failure, and normal short output.

### Milestone 7 — Context-aware compaction

**Goal:** compact only when the live next request approaches its usable context
budget.

#### Build

- Base decisions on estimated next-request context, not cumulative session
  usage.
- Preserve current task, important files, unresolved errors, and user intent.
- Report what was compacted and why.

#### Acceptance checks

- A long historical session with a small next request does not compact merely
  because its lifetime usage is high.
- A near-limit request compacts before it exceeds the configured context
  budget.

### Milestone 8 — Release reliability and documentation

**Goal:** the documented product matches tested behaviour.

#### Build

- Fix the existing Clippy failure in terminal output formatting.
- Make mock parity scenarios runnable on Windows or document an intentional
  platform-specific alternative.
- Update `INDEX.md`, `CLI-ROADMAP.md`, feature documentation, and testing
  instructions to point to this plan and actual Oog commands.
- Add CI for format, tests, and Clippy once the checks are green locally.

#### Acceptance checks

- `cargo fmt --check`, workspace tests, and Clippy are green.
- The mock/smoke harness executes at least one deterministic scenario in the
  supported development environment.
- Every command shown in `/help` is documented and tested.

## Deferred deliberately

- Full-screen TUI or alternate-screen renderer.
- Caveman engine/proxy integration.
- Cloud sync, accounts, collaborative session storage, and a database.
- New specialist commands before a measured need exists.
- Payload-specific compression algorithms before local evidence identifies the
  costly payload class.

## Working method

For each milestone:

1. Implement the smallest coherent slice.
2. Add the focused regression test before moving on.
3. Build and run the relevant CLI tests.
4. Let the user manually test visible terminal behaviour.
5. Update this document's current-baseline section when the milestone is
   complete.
