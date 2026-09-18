# UX Improvements Specification for Oog Code

**Version**: 1.0
**Date**: 2025-01-08
**Status**: Interview Complete - Ready for Implementation

## Executive Summary

This specification captures comprehensive UX improvements for the Oog Code CLI terminal renderer, based on in-depth technical interviews covering 10 major improvement areas. The specification prioritizes four critical areas identified by the user:

1. **Flow & Feedback** - Turn structure, active feedback indicators
2. **Output Management** - Smart collapsing, expandable sections
3. **Interaction Model** - Inline suggestions, interruptibility
4. **Context Awareness** - Dynamic prompts, session momentum

## Table of Contents

1. [Turn Structure Improvements](#1-turn-structure-improvements)
2. [Active Feedback System](#2-active-feedback-system)
3. [Smart Output Management](#3-smart-output-management)
4. [Interaction Enhancements](#4-interaction-enhancements)
5. [Keyboard Shortcuts](#5-keyboard-shortcuts)
6. [Context Awareness](#6-context-awareness)
7. [Tool Transparency](#7-tool-transparency)
8. [Error UX Improvements](#8-error-ux-improvements)
9. [Output Quality](#9-output-quality)
10. [Optional Modes](#10-optional-modes)

---

## 1. Turn Structure Improvements

### 1.1 Visual Separators Between Turns

**Requirement**: Add clear visual separators between conversation turns to improve scanability and context tracking.

**Implementation**:
```
─────────────────────────────────────────────────────────
```

**Width**: Full terminal width (detected via crossterm)

**Placement**:
- 2 newlines before separator
- 1 newline after separator
- Appears after each complete user→assistant→user cycle

**Interrupted Turn Handling**:
When a user interrupts streaming (Ctrl+C), use different separator style to indicate incomplete state:

```
╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌
```

**Rationale**: Visual indicator (`┄` vs `─`) helps user track conversation integrity and identify partial responses.

### 1.2 Turn Anchoring

**Requirement**: Add timestamp and turn count to each turn for reference and navigation.

**Format**:
```
[Turn #3] 2025-01-08 14:32:15
```

**Placement**:
- Above separator (2 lines before)
- Color: Gray
- Font: Regular (not bold)

**Turn Counting**:
- Increment after each user message
- Persist across session
- Reset on `/session new`

**Navigation Integration**:
- Press `g` + turn number to jump to that turn
- Example: `g3` jumps to turn #3

---

## 2. Active Feedback System

### 2.1 Processing Indicator

**Requirement**: Show clear visual indicator when Minseo is processing user input.

**Display**:
```
● Processing...
```

**Color**: Purple (RGB 189,147,249)

**Placement**:
- Inline with user input area
- Appears immediately after user submits message
- Disappears when first token arrives

**Animation**:
- **Pulse on stream** (user selection)
- Animate only during streaming content
- Pulse pattern: `●` → `⊙` → `●` (200ms cycle)
- Static during non-streaming operations

**Rationale**: Animation during streaming provides active feedback without being distracting during steady-state operations.

### 2.2 Streaming Cursor

**Requirement**: Add visual cursor at generation point during streaming responses.

**Display**: `▌` (U+258C LEFT HALF BLOCK)

**Placement**:
- At end of currently streamed content
- Disappears when generation complete

**Animation**:
- **Pulse pattern** (user selection)
- Cursor pulses during generation, disappears when idle
- Pulse cycle: visible (500ms) → invisible (300ms) → visible

**Performance**:
- Update cursor independent of token arrival
- Use separate thread/timer for pulse animation

**Rationale**: Pulse pattern balances visual feedback with rendering overhead, avoiding character-level performance cost.

---

## 3. Smart Output Management

### 3.1 Auto-Collapse System

**Requirement**: Automatically collapse verbose sections to maintain clean terminal while preserving access to full content.

**Per-Type Thresholds** (user selection):
- **Tool output**: 20 lines (tools often produce verbose output)
- **Praying sections**: 30 lines (praying typically brief but can be detailed)
- **Code blocks**: 40 lines (code needs more context)
- **Other content**: 50 lines (default threshold)

**Collapse Indicator**:
```
[PRAYING] [... 25 more lines ...] (Press Ctrl+K to expand)
```

**Auto-Expand Behavior** (user selection):
- Latest response always expanded (focus current)
- Pinned sections remain expanded (user control)
- Other sections auto-collapse after next response arrives

**Pinning Interface**:
- Press `p` to pin/unpin current section
- Pinned sections show `📌` prefix in header
- Pinned state persists for session only

### 3.2 Expandable Details

**Requirement**: Support expandable/collapsible sections for verbose content.

**Toggle Shortcut**: `Ctrl+K` (consistent with existing UX)

**Visual States**:

**Collapsed**:
```
● [RESULT] [... 42 more lines ...] (Press Ctrl+K to expand)
```

**Expanded**:
```
● [RESULT]
Full content here...
(Press Ctrl+K to collapse)
```

**State Persistence** (user selection):
- Session only - resets on CLI restart
- Fresh start every session balances consistency with freshness
- Pin state persists within session only

### 3.3 Pagination

**Requirement**: Implement smart pagination for very long content.

**Triggers** (user selection):
- **Content-aware**: Only paginate if content is homogenous (e.g., long list)
- **User preference**: `paginate me less` vs `paginate everything`

**Content Detection**:
- Lists (bullet points, numbered)
- Repetitive structures (similar lines)
- Code blocks over threshold
- Raw tool output

**Pagination Format**:
```
[Page 1/2 (45 lines)]
Navigation: Ctrl+D (down), Ctrl+U (up), Q to quit
```

**User Preference Command**:
```
/pagination [strict|relaxed|off]
```

**Default**: Relaxed (content-aware pagination)

---

## 4. Interaction Enhancements

### 4.1 Inline Suggestions

**Requirement**: Show actionable suggestions after [RESULT] sections to improve workflow continuity.

**Suggestion Logic** (user selection):
- **Primary**: Tool-chain logic (what naturally comes next)
- **Secondary**: Git-state awareness when applicable
- **Maximum**: 3 suggestions at a time

**Tool-Chain Examples**:
- After file edit → suggest testing/running
- After file write → suggest reading/committing
- After bash completion → suggest next logical command

**Git-State Examples**:
- Repo dirty → suggest commit/stash
- Repo clean → suggest push/branch
- New files → suggest add

**Display Format**:
```
● [RESULT]
Response content...

💡 Suggested next steps:
  1. Test the changes → /run
  2. Commit → /git commit
  3. Review → /read src/main.rs
```

**Interaction**:
- Press number key (1-3) to execute suggestion
- Press `s` to show suggestions again
- Press `Esc` to dismiss suggestions

### 4.2 Interruptibility

**Requirement**: Gracefully handle user interruption during streaming.

**Ctrl+C Behavior** (user selection):
- **First press**: Keep partial response
- **Second press within 2 seconds**: Clean rollback
- **Third press**: Force exit

**Partial Response Handling** (user selection):
- Keep all output received so far
- Add visual indicator using `┄` separator
- Show `[Interrupted]` marker at end of partial content

**Format**:
```
╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌
[Interrupted] Press Enter to continue or Ctrl+C to exit
```

**Rationale**: Preserves partial results for reference while giving user control over cleanup.

---

## 5. Keyboard Shortcuts

### 5.1 Core Shortcuts

| Shortcut | Action | Scope |
|----------|--------|-------|
| `t` | Toggle expand/collapse current section | Section |
| `d` | Mark section as done (collapse + dim) | Section |
| `Ctrl+C` | Interrupt streaming (1x = keep, 2x = rollback) | Global |
| `j` | Jump to next section | Navigation |
| `k` | Jump to previous section | Navigation |
| `p` | Pin/unpin current section | Section |
| `g` + number | Jump to turn # | Navigation |
| `s` | Show suggestions after [RESULT] | Contextual |
| `1-3` | Execute suggestion # | Contextual |

### 5.2 Keyboard Buffer Handling

**Requirement**: Preserve user input buffer when shortcuts are triggered.

**Behavior** (user selection): Smart restore

**Implementation**:
- Shortcuts clear pending input
- Show prompt: `[Press Tab to restore "your text here"]`
- User has 10 seconds to press Tab
- Buffer restores if Tab pressed within timeout
- Buffer discarded if timeout expires

**Rationale**: Prevents accidental sends while allowing recovery of in-progress composition.

### 5.3 Shortcut Help

**Command**: `/shortcuts` or `?`

**Display**:
```
Keyboard Shortcuts:
  t          Toggle section expand/collapse
  d          Mark section as done
  p          Pin/unpin section
  j/k        Next/previous section
  g#         Jump to turn #
  Ctrl+C     Interrupt (1x=keep, 2x=clear, 3x=exit)
  Ctrl+K     Expand/collapse section
  Ctrl+D/U   Pagination down/up
  s          Show suggestions
  1-3        Execute suggestion
```

---

## 6. Context Awareness

### 6.1 Dynamic Prompt

**Requirement**: Show different input prompts based on current state.

**States**:
- **Default**: `oog>`
- **Streaming**: `● Processing...` (no input prompt)
- **Waiting for approval**: `● waiting for approval>`

**Timeout Behavior** (user selection):
- **5 minute timeout** for "waiting for approval" state
- After timeout: `[Timeout - resuming normal mode]`
- Returns to default prompt

**Rationale**: Prevents confusion if user forgets about approval state while respecting deep work sessions.

### 6.2 Session Momentum Indicator

**Requirement**: Show metrics that indicate "productive flow" during session.

**Metrics** (user selection):
- **File modifications**: Count of files modified in session
- **Task completion**: Track pending vs completed action items

**Display**:
```
● [RESULT]
Response...

[Momentum: 5 files modified, 12 actions completed]
```

**Placement**:
- Below turn separator
- Color: Green
- Font: Dim

**Calculation**:
- File modifications: Tracked via file write/edit tools
- Task completion: Parses [ACTION] → [RESULT] pairs

**Update Frequency**: Refreshes after each complete turn

---

## 7. Tool Transparency

### 7.1 Intent Preview

**Requirement**: Show what tool will do before execution.

**Display Format** (user selection): Semantic description

**Examples**:
```
[Bash] Running: cargo test --release
[Read] Viewing: src/main.rs:1-50
[Edit] Modifying: src/main.rs (add function calculate_sum)
[Write] Creating: tests/integration_test.rs
```

**Implementation**:
- Parse tool arguments to extract semantic intent
- Fallback to truncated args if parsing fails
- Max length: 80 characters

**Placement**:
- Above tool output
- Color: Gray
- Font: Regular

### 7.2 Tool Execution Feedback

**Requirement**: Show clear start and end of tool execution.

**Start**:
```
● [TOOL] Running command...
```

**Success**:
```
✓ Command completed in 2.3s
```

**Error**:
```
✗ Command failed (exit code 1)
```

**Timing**: Show execution time for all tools

---

## 8. Error UX Improvements

### 8.1 Error Display Format

**Requirement**: Show errors with appropriate context and actionable guidance.

**Format** (user selection): Context window + suggested fix

**Structure**:
```
✗ Error: File not found (src/missing.rs)

Context:
   42 |   let file = read_file("src/missing.rs");
      |                      ^^^^^^^^^^^^^^^^^^^^ file not found

💡 Suggested fix:
   Check the file path or create the file:
   → /write src/missing.rs
   → /glob "*.rs" (find Rust files)
```

**Context Window Size**: Last 3 relevant lines

**Error Color**: Red

**Suggestion Color**: Yellow

### 8.2 Actionable Error Messages

**Requirement**: Every error must include at least one actionable suggestion.

**Suggestion Types**:
- Command to fix (e.g., `/write <file>`)
- Documentation link (e.g., `See: docs.md#section`)
- Troubleshooting steps
- Example command

**Error Categories**:
- Filesystem errors (not found, permission denied)
- API errors (rate limit, auth failure)
- Tool errors (non-zero exit, timeout)
- Parse errors (invalid input, malformed args)

---

## 9. Output Quality

### 9.1 Copy-Friendly Formatting

**Requirement**: Ensure terminal output copies cleanly to clipboard.

**Implementation** (user selection): Smart strip

**Behavior**:
- Auto-detect when user selects text
- Strip ANSI codes from clipboard content
- Preserve line breaks and indentation
- Preserve code block formatting

**Technical Implementation**:
- Use OSC 52 escape sequence for clipboard integration
- Strip ANSI color codes using regex: `\x1b\[[0-9;]*m`
- Preserve structural formatting (newlines, tabs)

### 9.2 Smart Wrapping

**Requirement**: Wrap long lines intelligently based on content type.

**Rules**:
- **Code blocks**: No wrap (preserve original formatting)
- **Tables**: Wrap at column boundaries
- **Long URLs**: Wrap at path separators (`/`, `.`)
- **Regular text**: Wrap at word boundaries

**Terminal Width Detection**:
- Use crossterm to detect terminal size
- Update on resize event
- Re-wrap existing content if width changes significantly

### 9.3 Truncation for Long Content

**Requirement**: Truncate overly long lines with ellipsis for readability.

**Threshold**: 120 characters

**Format**:
```
This is a very long line that exceeds the threshold... (120 more chars)
```

**Interaction**: Press `e` to expand truncated line

---

## 10. Optional Modes

### 10.1 Compact Mode

**Requirement**: Provide compact mode for smaller terminals or power users.

**Trigger** (user selection):
- **Session-scoped by default**
- **Auto-adaptive**: Enable on terminals < 40 lines height
- **Option to persist**: User can choose to save to config

**Compact Mode Changes**:
- Remove extra blank lines (2 before → 1 before)
- Collapse all sections by default
- Shorter separators (`──` instead of `────────`)
- Hide momentum indicator
- Inline suggestions (always shown, never dismissed)

**Toggle Command**:
```
/compact [on|off|auto]
```

**Persistence** (user selection):
- Session-scoped by default
- Option to persist in config
- Easy experimentation without commitment

### 10.2 Quiet Mode

**Requirement**: Reduce noise for experienced users.

**Quiet Mode Changes**:
- Hide processing indicator
- Hide momentum indicator
- Hide suggestions (unless requested with `s`)
- Minimal separators

**Toggle Command**:
```
/quiet [on|off]
```

### 10.3 Verbose Mode

**Requirement**: Show all details for debugging.

**Verbose Mode Changes**:
- Show full tool args (not semantic descriptions)
- Show execution time for all operations
- Show token usage per response
- Show internal state changes

**Toggle Command**:
```
/verbose [on|off]
```

---

## Implementation Priority

### Phase 1: Foundation (Week 1)
1. Turn structure improvements (separators, anchoring)
2. Active feedback system (processing indicator, streaming cursor)
3. Keyboard shortcuts core (t, d, Ctrl+C, j, k)

### Phase 2: Output Management (Week 2)
1. Auto-collapse system with per-type thresholds
2. Expandable sections with Ctrl+K toggle
3. Smart pagination (content-aware)

### Phase 3: Interaction (Week 3)
1. Inline suggestions (tool-chain + git-aware)
2. Interruptibility with partial response handling
3. Keyboard buffer smart restore

### Phase 4: Context & Polish (Week 4)
1. Dynamic prompts with timeout
2. Session momentum indicator
3. Tool transparency (semantic descriptions)
4. Error UX improvements (context + suggestions)
5. Copy-friendly formatting
6. Optional modes (compact, quiet, verbose)

---

## Technical Considerations

### Performance
- **Streaming cursor**: Use separate thread for pulse animation to avoid blocking token stream
- **Auto-collapse**: Defer collapse calculation until after response complete
- **Pagination**: Implement lazy loading for large content

### State Management
- **Section state**: Store in memory, reset on session restart
- **Pin state**: Persist for session only
- **User preferences**: Store in `.claude.json`

### Compatibility
- **Terminal size**: Use crossterm for cross-platform detection
- **ANSI codes**: Test on Windows Terminal, iTerm2, Linux TTY
- **Clipboard**: OSC 52 for terminal integration, fallback to system clipboard

### Accessibility
- **Color blind support**: Ensure contrast ratios meet WCAG AA
- **Screen readers**: Preserve semantic structure in markdown
- **Keyboard navigation**: All features accessible via keyboard

---

## Testing Checklist

### Functional Testing
- [ ] Turn separators appear correctly
- [ ] Interrupted turns use different separator style
- [ ] Processing indicator appears/disappears at correct times
- [ ] Streaming cursor pulses during generation
- [ ] Auto-collapse works with per-type thresholds
- [ ] Ctrl+K toggles expand/collapse
- [ ] Pin/unpin works with `p` key
- [ ] Inline suggestions appear after [RESULT]
- [ ] Suggestions execute on number press
- [ ] Ctrl+C interrupt handling (1x, 2x, 3x)
- [ ] Keyboard buffer restore works with Tab
- [ ] Pagination triggers correctly
- [ ] Dynamic prompt changes based on state
- [ ] Momentum indicator updates after turns
- [ ] Tool intent preview shows semantic descriptions
- [ ] Errors show context + suggestions
- [ ] Copy strips ANSI codes
- [ ] Compact mode activates on small terminals
- [ ] All keyboard shortcuts work

### Visual Testing
- [ ] Color rendering correct on all terminals
- [ ] Separator lines span full width
- [ ] Collapsed sections show correct line count
- [ ] Progress bar/pulse animations smooth
- [ ] Suggestions format cleanly
- [ ] Error context aligns correctly
- [ ] Copy-paste preserves formatting

### Integration Testing
- [ ] Works with existing renderer (/renderer on/off)
- [ ] Compatible with slash commands
- [ ] Session resume preserves state correctly
- [ ] REPL mode functions normally
- [ ] One-shot prompt mode unaffected

---

## Success Metrics

### Quantitative
- **Turn recognition**: 90% of users can identify conversation turns within 5 seconds
- **Navigation efficiency**: 50% reduction in time to jump between sections
- **Error recovery**: 70% of errors resolved without external documentation
- **Suggestion adoption**: 30% of suggestions acted upon (indicates relevance)

### Qualitative
- **Flow state**: Users report fewer interruptions during coding sessions
- **Confidence**: Users feel more confident interrupting and resuming work
- **Clarity**: Users understand what the system is doing at all times
- **Control**: Users feel in control of output density and detail level

---

## Open Questions

1. **Mobile/small terminal support**: Should we have an even more aggressive "mobile" mode?
2. **Multi-session awareness**: Should momentum indicator track across sessions?
3. **Suggestion learning**: Should the system learn from user's dismissed suggestions?
4. **Collaborative features**: Should we support sharing/collapsing sections for screen sharing?
5. **Theme support**: Should colors be configurable beyond the Minseo palette?

---

## Appendix A: ANSI Color Codes Reference

```rust
// Current Minseo Color Palette
MinseoColor::Green   => "\x1b[32m"                    // Success
MinseoColor::Red     => "\x1b[31m"                    // Error
MinseoColor::Gray    => "\x1b[90m"                    // Secondary
MinseoColor::Purple  => "\x1b[38;2;189;147;249m"      // Brand (RGB true color)
MinseoColor::White   => "\x1b[38;2;255;255;255m"      // Bullets (RGB true color)
MinseoColor::Yellow  => "\x1b[38;2;255;255;0m"        // Bullets (RGB true color)

// Reset
MinseoColor::reset() => "\x1b[0m"

// Bold
"\x1b[1m" ... "\x1b[22m"

// Dim
"\x1b[2m" ... "\x1b[22m"
```

---

## Appendix B: Keyboard Shortcut Reference Card

```
╔══════════════════════════════════════════════════════════════════╗
║                    Oog Code Keyboard Shortcuts                   ║
╠══════════════════════════════════════════════════════════════════╣
║ Navigation                                                       ║
║  j/k              Next/previous section                          ║
║  g#               Jump to turn #                                 ║
║  Ctrl+D/U         Pagination down/up                             ║
╟──────────────────────────────────────────────────────────────────╢
║ Section Control                                                  ║
║  t                Toggle expand/collapse                         ║
║  d                Mark as done (collapse + dim)                  ║
║  p                Pin/unpin section                              ║
║  Ctrl+K           Expand/collapse detailed content               ║
╟──────────────────────────────────────────────────────────────────╢
║ Global                                                           ║
║  Ctrl+C (1x)      Interrupt (keep partial)                       ║
║  Ctrl+C (2x)      Interrupt (clear partial)                     ║
║  Ctrl+C (3x)      Force exit                                    ║
║  Tab              Restore keyboard buffer                        ║
╟──────────────────────────────────────────────────────────────────╢
║ Contextual                                                        ║
║  s                Show suggestions after [RESULT]                ║
║  1-3              Execute suggestion #                           ║
║  e                Expand truncated line                          ║
╟──────────────────────────────────────────────────────────────────╢
║ Help                                                             ║
║  ? or /shortcuts   Show this reference card                      ║
╚══════════════════════════════════════════════════════════════════╝
```

---

**Document Status**: ✅ Complete - Ready for Implementation
**Next Steps**: Create detailed implementation plan with file-by-file changes, then begin Phase 1 development.
