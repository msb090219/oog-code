# Priority Fixes for New Renderer

## High Priority Fixes (Batch 1)

### 1. Fix Mutex unwrap() → expect()
**Files**: 9 component files
**Instances**: 62
**Action**: Replace all `.lock().unwrap()` with `.lock().expect("Mutex poisoned")`

**Script**:
```bash
# Run this in the project root
find crates/rusty-claude-cli/src/new_renderer/components -name "*.rs" -exec sed -i 's/\.lock()\.unwrap()/.lock().expect("Mutex poisoned")/g' {} \;
```

### 2. Add bounds to ErrorContext
**File**: `error_ux.rs`
**Issue**: Unbounded Vec growth
**Fix**: Add max limits to code_lines and suggestions

**Changes**:
```rust
pub const MAX_CODE_LINES: usize = 10;
pub const MAX_SUGGESTIONS: usize = 5;

pub fn add_code_line(&mut self, line: impl Into<String>) -> &mut Self {
    if self.code_lines.len() < MAX_CODE_LINES {
        self.code_lines.push(line.into());
    }
    self
}

pub fn add_suggestion(&mut self, suggestion: impl Into<String>) -> &mut Self {
    if self.suggestions.len() < MAX_SUGGESTIONS {
        self.suggestions.push(suggestion.into());
    }
    self
}
```

### 3. Add reset mechanism to SessionMetrics
**File**: `momentum.rs`
**Issue**: No way to reset metrics
**Fix**: Already exists - `reset_session()` method
**Status**: ✅ Already implemented

## Medium Priority Fixes (Batch 2)

### 4. Fix redundant clone warning
**File**: `tool_transparency.rs:111`
**Issue**: `error.clone().red()` on &str
**Fix**: Change to `error.red()` (no clone needed)

### 5. Add Copy trait to PromptContext
**File**: `dynamic_prompt.rs`
**Status**: ✅ Already fixed

## Testing Fixes

### 6. Fix test compilation issues
**Issues**: Unix-specific code on Windows
**Files**: Test files with `std::os::unix`
**Status**: Known issue, not critical for renderer

---

## Fix Execution Order

1. ✅ Phase 4 components - Fixed PromptContext
2. ⏳ Batch 1: Mutex unwrap() fixes (62 instances)
3. ⏳ Batch 2: ErrorContext bounds
4. ⏳ Batch 3: Redundant clone fix

---

## After Fixes

Run tests:
```bash
cargo check
cargo test --lib new_renderer
cargo build --release
```

Expected: Zero errors, zero warnings
