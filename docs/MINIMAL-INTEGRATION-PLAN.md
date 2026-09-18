# Retired: Minimal TUI Integration Plan

> **Retired.** This TUI integration path was abandoned in favor of the inline
> terminal CLI. See [planning/CLI-ROADMAP.md](planning/CLI-ROADMAP.md).

**Philosophy**: Simple outside, powerful inside.
**Goal**: Get status bar displaying in LiveCli REPL.

---

## Current State

**What's Built:**
- ✅ EventLoop (tokio::select!, 60fps throttling)
- ✅ LayeredRenderer (3-layer system)
- ✅ StatusBar (smart truncation, adaptive)
- ✅ OutputLayer (markdown, collapsible sections, modes)
- ✅ Theme system (color capability detection)
- ✅ RenderEvent enum (15 event types)
- ✅ UiState (centralized state)

**What's NOT Integrated:**
- ❌ None of this is connected to LiveCli
- ❌ main.rs still uses rustyline + inline stdout
- ❌ Status bar not visible during operation
- ❌ No RenderEvents being sent

---

## Integration Strategy: Incremental

**Principle**: One feature at a time, test it works, then move to the next.

### Phase 1: Status Bar Integration (1-2 hours)

**Goal**: See the status bar at the bottom of the terminal during REPL.

**Steps:**

1. **Add TUI infrastructure to LiveCli**
   ```rust
   // In main.rs, struct LiveCli
   pub struct LiveCli {
       // ... existing fields ...

       // TUI infrastructure (NEW)
       ui_state: Arc<Mutex<UiState>>,
       render_tx: mpsc::Sender<RenderEvent>,
       _event_loop_handle: tokio::task::JoinHandle<()>,
   }
   ```

2. **Initialize in LiveCli::new()**
   ```rust
   // Create channels for RenderEvents
   let (render_tx, render_rx) = mpsc::channel(100);

   // Create initial UiState
   let ui_state = Arc::new(Mutex::new(UiState::new()));

   // Spawn event loop task
   let event_loop_handle = tokio::spawn(async move {
       let mut event_loop = EventLoop::new(render_rx);
       event_loop.run().await;
   });

   // Store in LiveCli
   Self {
       // ... existing fields ...
       ui_state,
       render_tx,
       _event_loop_handle: event_loop_handle,
   }
   ```

3. **Send RenderEvents during streaming**
   ```rust
   // When token count updates
   self.render_tx.send(RenderEvent::UsageUpdate {
       usage: updated_usage,
       timestamp: Instant::now(),
   }).await.ok();

   // When streaming starts
   self.render_tx.send(RenderEvent::TurnStart {
       turn_number: self.turn_counter,
       timestamp: Instant::now(),
   }).await.ok();
   ```

4. **StatusBar integration in EventLoop**
   ```rust
   // In EventLoop::run(), when rendering
   let status_bar = StatusBar::new();
   status_bar.render(&ui_state)?;
   ```

**Success Criteria:**
- Status bar visible at bottom of terminal
- Token count updates live during streaming
- Model name displays correctly
- Cost updates in real-time

---

## Why This Approach?

### 1. Minimal Risk
- We're adding TUI as a parallel system, not replacing anything
- Existing REPL continues to work
- If status bar breaks, REPL still functions

### 2. Testable
- We can verify status bar works before moving to complex features
- Each step builds on the previous
- Easy to debug if something goes wrong

### 3. Aligned with Philosophy
- **Simple outside**: Status bar looks minimal
- **Powerful inside**: Smart truncation, adaptive behavior

---

## After Phase 1

Once status bar is working:

1. **Activate output layer** for markdown rendering (Phase 2)
2. **Activate input layer** for async input (Phase 6)
3. **Add overlay layer** for slash command suggestions (Phase 4)

Each following the same pattern: integrate, test, then move on.

---

## Key Files to Modify

**Primary:**
- `crates/rusty-claude-cli/src/main.rs` - Add TUI infrastructure to LiveCli

**Supporting:**
- `crates/rusty-claude-cli/src/tui/event_loop.rs` - Integrate StatusBar rendering
- `crates/rusty-claude-cli/src/tui/layers/output.rs` - Will be activated in Phase 2

---

## What We're NOT Doing

- ❌ Replacing rustyline (yet - that's Phase 6)
- ❌ Full ratatui widget system (staying with custom renderer)
- ❌ Alternate screen mode (keeping inline REPL)
- ❌ Big bang integration (incremental only)

---

## Summary

**This session**: Get status bar displaying in LiveCli
**Next session**: Activate output layer for markdown rendering
**Future**: Input layer, overlay, remaining features

**One feature at a time. Test it works. Move to the next.**

---

*Last Updated: 2026-04-25*
