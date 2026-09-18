// Suggestion Manager for Dropdown UI
// Manages dropdown state, rendering, and key event handling

use crate::completion::{CompletionEngine, Suggestion, SuggestionKind};
use crate::render::ColorTheme;
use crossterm::{
    cursor,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
    QueueableCommand,
};
use std::io::{self, Write};

/// Dropdown state
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DropdownState {
    Hidden,
    Visible {
        suggestions: Vec<Suggestion>,
        selected_index: usize,
        viewport_start: usize,
        anchor: (u16, u16),
    },
}

impl DropdownState {
    pub fn is_visible(&self) -> bool {
        matches!(self, DropdownState::Visible { .. })
    }

    pub fn get_selected_index(&self) -> Option<usize> {
        match self {
            DropdownState::Visible { selected_index, .. } => Some(*selected_index),
            _ => None,
        }
    }
}

/// Action to take after handling a key event
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DropdownAction {
    None,
    Select(usize),
    Dismiss,
    Insert(String),
}

/// Suggestion manager
pub struct SuggestionManager {
    dropdown: DropdownState,
    engine: CompletionEngine,
    theme: ColorTheme,
    max_visible_items: usize,
}

impl SuggestionManager {
    pub fn new(theme: ColorTheme) -> Self {
        Self {
            dropdown: DropdownState::Hidden,
            engine: CompletionEngine::new(),
            theme,
            max_visible_items: 8,
        }
    }

    /// Update suggestions based on current input
    pub fn update(&mut self, input: &str, cursor_pos: usize) {
        let context = crate::completion::CompletionContext::new(input.to_string(), cursor_pos);
        let suggestions = self.engine.get_completions(&context);

        if suggestions.is_empty() {
            self.hide();
        } else {
            // Get current cursor position for anchor
            if let Ok((col, row)) = cursor::position() {
                self.show(suggestions, (col, row));
            }
        }
    }

    /// Handle a key event
    pub fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> DropdownAction {
        if !self.dropdown.is_visible() {
            return DropdownAction::None;
        }

        match key.code {
            crossterm::event::KeyCode::Up => self.move_selection(-1),
            crossterm::event::KeyCode::Down => self.move_selection(1),
            crossterm::event::KeyCode::Enter => self.confirm_selection(),
            crossterm::event::KeyCode::Esc => {
                self.hide();
                DropdownAction::Dismiss
            }
            _ => DropdownAction::None,
        }
    }

    /// Show dropdown with suggestions
    pub fn show(&mut self, suggestions: Vec<Suggestion>, anchor: (u16, u16)) {
        self.dropdown = DropdownState::Visible {
            suggestions,
            selected_index: 0,
            viewport_start: 0,
            anchor,
        };
    }

    /// Hide dropdown
    pub fn hide(&mut self) {
        self.dropdown = DropdownState::Hidden;
    }

    /// Move selection up or down
    fn move_selection(&mut self, direction: isize) -> DropdownAction {
        if let DropdownState::Visible {
            ref mut selected_index,
            ref suggestions,
            ref mut viewport_start,
            ..
        } = self.dropdown
        {
            let len = suggestions.len();
            if len == 0 {
                return DropdownAction::None;
            }

            let new_index = if direction > 0 {
                (*selected_index + 1) % len
            } else {
                selected_index.saturating_sub(1)
            };

            *selected_index = new_index;

            // Adjust viewport if needed
            if new_index < *viewport_start {
                *viewport_start = new_index;
            } else if new_index >= *viewport_start + self.max_visible_items {
                *viewport_start = new_index.saturating_sub(self.max_visible_items - 1);
            }

            DropdownAction::None
        } else {
            DropdownAction::None
        }
    }

    /// Confirm current selection
    fn confirm_selection(&self) -> DropdownAction {
        if let DropdownState::Visible {
            suggestions,
            selected_index,
            ..
        } = &self.dropdown
        {
            if let Some(suggestion) = suggestions.get(*selected_index) {
                return DropdownAction::Insert(suggestion.replacement.clone());
            }
        }
        DropdownAction::None
    }

    /// Render the dropdown
    pub fn render(&self) -> io::Result<()> {
        if let DropdownState::Visible {
            suggestions,
            selected_index,
            viewport_start,
            anchor,
        } = &self.dropdown
        {
            let (col, row) = *anchor;
            let (_, terminal_height) = terminal::size()?;

            // Calculate dropdown position (2 lines below cursor)
            let dropdown_row = row
                .saturating_add(2)
                .min(terminal_height.saturating_sub(10));

            // Calculate visible items
            let max_items = self
                .max_visible_items
                .min(terminal_height.saturating_sub(dropdown_row) as usize);
            let visible_suggestions: Vec<_> = suggestions
                .iter()
                .skip(*viewport_start)
                .take(max_items)
                .enumerate()
                .collect();

            if visible_suggestions.is_empty() {
                return Ok(());
            }

            // Find max width for alignment
            let max_width = visible_suggestions
                .iter()
                .map(|(_, s)| s.display.len())
                .max()
                .unwrap_or(20)
                .max(30);

            let mut stdout = io::stdout();

            // Render dropdown box
            for (i, suggestion) in visible_suggestions {
                let actual_index = *viewport_start + i;
                let is_selected = actual_index == *selected_index;

                // Move to dropdown position
                stdout.queue(cursor::MoveTo(col, dropdown_row + i as u16))?;

                // Set background for selected item
                if is_selected {
                    stdout.queue(SetBackgroundColor(Color::Rgb {
                        r: 40,
                        g: 120,
                        b: 200,
                    }))?;
                }

                // Render item
                let display_text = format!(
                    "{:width$} {}",
                    suggestion.display,
                    suggestion.kind.kind_as_str(),
                    width = max_width + 2
                );

                if is_selected {
                    stdout.queue(SetForegroundColor(Color::Black))?;
                    stdout.queue(Print(display_text))?;
                    stdout.queue(SetForegroundColor(Color::Reset))?;
                } else {
                    stdout.queue(Print(display_text))?;
                }

                stdout.queue(ResetColor)?;
                stdout.queue(cursor::MoveTo(col, dropdown_row + i as u16))?;

                // Render description if available
                if let Some(desc) = &suggestion.description {
                    let desc_text = format!("  {desc}");
                    stdout.queue(Print(desc_text))?;
                }

                stdout.queue(ResetColor)?;
            }

            stdout.flush()?;
        }

        Ok(())
    }

    /// Update sessions in completion engine
    pub fn update_sessions(&mut self, sessions: Vec<String>) {
        self.engine.update_sessions(sessions);
    }

    /// Update history in completion engine
    pub fn update_history(&mut self, history: Vec<String>) {
        self.engine.update_history(history);
    }
}

impl SuggestionKind {
    pub fn kind_as_str(&self) -> &str {
        match self {
            SuggestionKind::Command => "[Cmd]",
            SuggestionKind::Argument => "[Arg]",
            SuggestionKind::SessionId => "[Sess]",
            SuggestionKind::ModelName => "[Model]",
            SuggestionKind::PermissionMode => "[Mode]",
            SuggestionKind::ThemeName => "[Theme]",
            SuggestionKind::HistoryEntry => "[Hist]",
        }
    }
}
