Below is a **concise but production-ready PRD** for the input area system you are envisioning. It is written to align with your existing Oog Code architecture and CLI constraints.

---

# **Product Requirements Document (PRD)**

## **Feature: Persistent Async Input Composer with Slash Command System**

---

## **1. Overview**

This feature introduces a **persistent, non-blocking input composer** for Oog Code, alongside an **interactive slash-command suggestion system**.

The goal is to transform the current REPL input into a **modern, fluid, messenger-like interface**, where:

* Users can type while the assistant is generating
* Commands are discoverable via `/`
* Interaction feels responsive, continuous, and interruption-tolerant

---

## **2. Objectives**

### Primary Objectives

* Eliminate input blocking during assistant streaming
* Improve command discoverability and usability
* Enable continuous workflow without waiting for responses

### Secondary Objectives

* Reduce cognitive friction in long sessions
* Improve perceived performance of the system
* Align CLI UX with modern AI tools (Claude Code, Codex CLI)

---

## **3. User Experience**

### 3.1 Persistent Input Composer

The input area:

* Remains **fixed at the bottom** of the terminal
* Is always **editable**, regardless of assistant state
* Is visually separated from output

#### Example

```text
[ Assistant streaming output above... ]

────────────────────────────────────────────
› Refactor the parser into modules
────────────────────────────────────────────
```

---

### 3.2 Async Input While Streaming

While the assistant is generating:

* User can type freely
* Input is not overwritten by streaming output
* Cursor position remains stable

---

### 3.3 Message Queueing

If the user submits input during generation:

```text
› Add unit tests for edge cases
queued · will send after current response
```

System behaviour:

* Message is queued
* Automatically sent after current turn completes
* Multiple messages may be queued (FIFO)

---

### 3.4 Interruption Controls

| Action         | Behaviour                    |
| -------------- | ---------------------------- |
| `Enter`        | Submit / queue message       |
| `Shift+Enter`  | Insert newline               |
| `Esc` / Ctrl+C | Interrupt current generation |

---

### 3.5 Slash Command System

Typing `/` triggers a suggestion panel.

#### Example

```text
› /co
────────────────────────────────────────────
  /compact        Summarise conversation
  /config         Open configuration
  /cost           Show session cost
```

#### Behaviour

* Real-time filtering
* Keyboard navigation (↑ ↓)
* Enter/Tab to select
* Esc to dismiss

---

### 3.6 Command Metadata Display

Each command includes:

* Name
* Aliases (dimmed)
* Description

```text
/clear (reset, new)    Clear conversation history
```

---

### 3.7 Status Row (Optional but Recommended)

Display lightweight system state:

```text
opus · workspace-write · 38% context · main
```

---

## **4. Functional Requirements**

### 4.1 Input System

* Must support **non-blocking input**
* Must persist across streaming updates
* Must not be overwritten by output rendering
* Must maintain cursor position

---

### 4.2 Streaming Integration

* Output rendering must be **decoupled** from input rendering
* Streaming updates must **not reflow input area**
* Input must render on a separate buffer layer

---

### 4.3 Queue System

* FIFO message queue
* Visual feedback for queued messages
* Auto-dispatch after response completion
* Queue clears on session reset (`/clear`)

---

### 4.4 Slash Command Engine

* Trigger on `/`
* Filter dynamically
* Integrate with existing command registry
* Support aliases
* Support descriptions

---

### 4.5 Keyboard Handling

* Arrow navigation for suggestions
* Tab completion
* Escape handling for closing UI elements
* Interrupt signal handling

---

## **5. Non-Functional Requirements**

### Performance

* Input latency must be near-instant (<16ms perceived)
* No flickering during streaming

### Reliability

* Input must never be lost during rendering
* Queue must persist within session

### Usability

* Discoverable without documentation
* Minimal visual clutter
* Consistent with terminal constraints

---

## **6. Technical Design Considerations**

### 6.1 Rendering Model

Move from:

* Linear stdout rendering

To:

* **Layered terminal rendering**

  * Output layer (scrolling)
  * Input layer (fixed bottom)
  * Overlay layer (suggestions)

---

### 6.2 Suggested Stack (Rust)

* `crossterm` → cursor control, screen regions
* `tui` / `ratatui` (optional) → layout abstraction
* Custom renderer → for streaming + overlays

---

### 6.3 State Management

Maintain:

```rust
struct UiState {
    input_buffer: String,
    cursor_position: usize,
    suggestion_state: Option<SuggestionState>,
    message_queue: VecDeque<Message>,
    is_streaming: bool,
}
```

---

### 6.4 Integration Points

* REPL loop (`rustyline`) must be partially replaced or wrapped
* Slash command registry reused from existing system
* Streaming handler must emit events instead of directly printing

---

## **7. Edge Cases**

* Rapid Enter spam → queue overflow handling
* Interrupt during tool execution
* Very long input lines
* Terminal resize
* Suggestion panel overflow

---

## **8. Success Metrics**

* Reduced idle time between prompts
* Increased command usage
* Improved perceived responsiveness
* Reduced user frustration in long outputs

---

## **9. Future Extensions**

* Inline command preview (e.g. `/model sonnet`)
* Command categories (grouped UI)
* Fuzzy search for commands
* History-based suggestions
* Multi-line rich input editing
* AI-assisted command suggestions

---

## **10. Final Summary**

This feature upgrades Oog Code from a traditional REPL into a **modern, asynchronous conversational interface**, defined by:

* Persistent input
* Concurrent interaction
* Intelligent command discovery
* Fluid, interruption-friendly workflow

---

If you want, the next step would be to turn this into a **renderer architecture spec + implementation plan (with phases and code structure)** — which is where this becomes realistically buildable in your Rust codebase.
