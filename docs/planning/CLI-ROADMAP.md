# Terminal CLI roadmap

## Product boundary

Ship a dependable inline terminal coding assistant: a normal REPL with
scrollback, `rustyline` editing/history/completion, streamed Markdown, and
tool execution with permission prompts. No full-screen TUI, ratatui, custom
input event loop, message queue, or persistent status bar.

## 0. Establish a green, reproducible baseline

- Fix `build_runtime_runs_plugin_lifecycle_init_and_shutdown`; plugin init does
  not currently produce its expected lifecycle log.
- Remove the current compiler warnings and require `cargo clippy --all-targets
  --all-features -- -D warnings` to pass.
- Run the mock parity harness in addition to workspace tests.
- Restore usable Git metadata before relying on CLI Git workflows or commit
  references; this checkout's `.git` directory is empty.

**Done when:** workspace tests, Clippy, and the mock parity harness are green.

## 1. Close the real execution gaps

- Finish and test plugin install/enable/disable/uninstall lifecycle.
- Test config precedence (user, project, local) and document the final rules.
- Either implement real LSP JSON-RPC calls or label the current structured
  placeholder as unsupported; do not advertise it as complete.
- Set bounded output rules for tool results and model context so large files or
  command output cannot overwhelm the terminal or request budget.
- Validate session compaction and token/cost accounting against fixed fixtures.

**Done when:** every advertised non-stub tool has an end-to-end deterministic
test; unsupported tools are explicitly marked unsupported.

## 2. Make the existing CLI coherent

- Audit every visible slash command: implement it, hide it, or return one clear
  "not available" message. Do not leave parse-only commands presented as ready.
- Keep completion and help generated from the shared command registry.
- Make one-shot text and JSON output stable enough for scripts.
- Keep permission prompts, errors, tool start/end output, and interruption
  messages compact and consistent in ordinary terminal scrollback.

**Done when:** a user can discover, run, and recover from each advertised
workflow without reading source code.

## 3. Polish only where terminal use proves it needs polish

- Improve the existing streaming renderer for complete Markdown blocks and
  readable code/tool output.
- Add bounded, expandable-on-request output only if ordinary truncation proves
  insufficient.
- Add narrowly scoped commands such as history search or undo only after their
  underlying data and safety semantics are defined.

**Done when:** interactive use is clear in Windows Terminal, PowerShell, and a
typical Unix terminal without owning the terminal screen.

## 4. Release readiness

- Update README, features, parity, and command help from tested behavior.
- Add a CI job for the baseline checks.
- Publish a small smoke-test script using the mock service; keep the real API
  smoke test opt-in because it needs credentials.

## Explicitly not planned

Do not add ratatui, `src/tui/`, an alternate screen, a custom terminal event
loop, async typing during generation, or a replacement renderer experiment.
Reconsider only after the terminal-first CLI is reliable and real user
feedback establishes a concrete need.
