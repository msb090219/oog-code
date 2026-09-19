use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::BTreeSet;
use std::io::{self, IsTerminal, Write};
use std::time::Duration;

use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::{CmdKind, Highlighter};
use rustyline::hint::{Hint, Hinter};
use rustyline::history::DefaultHistory;
use rustyline::validate::Validator;
use rustyline::{
    Cmd, CompletionType, Config, Context, EditMode, Editor, Helper, KeyCode, KeyEvent, Modifiers,
};

use crate::completion::{CompletionContext, CompletionEngine, Suggestion};
use crate::render::ColorTheme;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReadOutcome {
    Submit(String),
    Cancel,
    Exit,
}

struct SlashCommandHelper {
    completions: Vec<String>,
    current_line: RefCell<String>,
    pub(crate) completion_engine: CompletionEngine,
}

impl SlashCommandHelper {
    fn new(completions: Vec<String>) -> Self {
        Self {
            completions: normalize_completions(completions),
            current_line: RefCell::new(String::new()),
            completion_engine: CompletionEngine::new(),
        }
    }

    fn reset_current_line(&self) {
        self.current_line.borrow_mut().clear();
    }

    fn current_line(&self) -> String {
        self.current_line.borrow().clone()
    }

    fn set_current_line(&self, line: &str) {
        let mut current = self.current_line.borrow_mut();
        current.clear();
        current.push_str(line);
    }

    fn set_completions(&mut self, completions: Vec<String>) {
        self.completions = normalize_completions(completions);
    }

    fn update_sessions(&mut self, sessions: Vec<String>) {
        self.completion_engine.update_sessions(sessions);
    }

    fn update_history(&mut self, history: Vec<String>) {
        self.completion_engine.update_history(history);
    }
}

impl Completer for SlashCommandHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let Some(prefix) = slash_command_prefix(line, pos) else {
            return Ok((0, Vec::new()));
        };

        // Use our intelligent completion engine
        let context = CompletionContext::new(line.to_string(), pos);
        let suggestions = self.completion_engine.get_completions(&context);

        if suggestions.is_empty() {
            // Fall back to basic completions
            let matches = self
                .completions
                .iter()
                .filter(|candidate| candidate.starts_with(prefix))
                .map(|candidate| Pair {
                    display: candidate.clone(),
                    replacement: candidate.clone(),
                })
                .collect();
            Ok((0, matches))
        } else {
            // Use intelligent suggestions
            let matches = suggestions
                .into_iter()
                .map(|s| Pair {
                    display: s.display,
                    replacement: s.replacement,
                })
                .collect();
            Ok((0, matches))
        }
    }
}

impl Hinter for SlashCommandHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Option<Self::Hint> {
        // Only show hints for slash commands at end of line
        if pos != line.len() || !line.starts_with('/') {
            return None;
        }

        // Get suggestions
        let context = CompletionContext::new(line.to_string(), pos);
        let suggestions = self.completion_engine.get_completions(&context);

        if let Some(first) = suggestions.first() {
            // Show the rest of the command as gray hint
            if let Some(rest) = first.replacement.strip_prefix(line) {
                if !rest.is_empty() {
                    return Some(rest.to_string());
                }
            }
        }

        None
    }
}

impl Highlighter for SlashCommandHelper {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        self.set_current_line(line);
        Cow::Borrowed(line)
    }

    fn highlight_char(&self, line: &str, _pos: usize, _kind: CmdKind) -> bool {
        self.set_current_line(line);
        false
    }
}

impl Validator for SlashCommandHelper {}
impl Helper for SlashCommandHelper {}

pub struct LineEditor {
    prompt: String,
    footer: String,
    composer_visible: bool,
    editor: Editor<SlashCommandHelper, DefaultHistory>,
}

impl LineEditor {
    #[must_use]
    pub fn new(prompt: impl Into<String>, completions: Vec<String>) -> Self {
        let config = Config::builder()
            .completion_type(CompletionType::Circular)
            .edit_mode(EditMode::Emacs)
            .build();
        let mut editor = Editor::<SlashCommandHelper, DefaultHistory>::with_config(config)
            .expect("rustyline editor should initialize");
        editor.set_helper(Some(SlashCommandHelper::new(completions)));
        editor.bind_sequence(KeyEvent(KeyCode::Char('J'), Modifiers::CTRL), Cmd::Newline);
        editor.bind_sequence(KeyEvent(KeyCode::Enter, Modifiers::SHIFT), Cmd::Newline);

        Self {
            prompt: prompt.into(),
            footer: String::new(),
            composer_visible: false,
            editor,
        }
    }

    pub fn set_footer(&mut self, footer: impl Into<String>) {
        self.footer = footer.into();
    }

    pub fn push_history(&mut self, entry: impl Into<String>) {
        let entry = entry.into();
        if entry.trim().is_empty() {
            return;
        }

        let _ = self.editor.add_history_entry(entry);
    }

    pub fn set_completions(&mut self, completions: Vec<String>) {
        if let Some(helper) = self.editor.helper_mut() {
            helper.set_completions(completions);
        }
    }

    pub fn update_sessions(&mut self, sessions: Vec<String>) {
        if let Some(helper) = self.editor.helper_mut() {
            helper.update_sessions(sessions);
        }
    }

    pub fn update_history(&mut self, history: Vec<String>) {
        if let Some(helper) = self.editor.helper_mut() {
            helper.update_history(history);
        }
    }

    /// Read a line with live dropdown that appears when typing "/"
    pub fn read_line_with_live_dropdown(&mut self) -> io::Result<ReadOutcome> {
        self.read_line_with_live_dropdown_and(|| Ok(()))
    }

    /// Read a line while allowing a caller to service background terminal work.
    ///
    /// The callback runs while the editor is idle, before the next key event is
    /// read. This keeps the input loop event-driven without adding another
    /// terminal dependency.
    pub fn read_line_with_live_dropdown_and<F>(&mut self, mut on_idle: F) -> io::Result<ReadOutcome>
    where
        F: FnMut() -> io::Result<()>,
    {
        // Try custom dropdown first, fall back to normal if it fails
        match self.try_read_with_dropdown(&mut on_idle) {
            Ok(result) => Ok(result),
            Err(_) => self.read_line(),
        }
    }

    fn try_read_with_dropdown<F>(&mut self, on_idle: &mut F) -> io::Result<ReadOutcome>
    where
        F: FnMut() -> io::Result<()>,
    {
        use crossterm::cursor;
        use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
        use crossterm::style;
        use crossterm::terminal::{self, disable_raw_mode, enable_raw_mode};
        use crossterm::QueueableCommand;
        use std::io::Write;

        enable_raw_mode()?;

        let mut input = String::new();
        let mut dropdown_visible = false;
        let mut suggestions = Vec::new();
        let mut selected_index = 0;

        self.render_line_and_dropdown(&input, &suggestions, selected_index, false)?;

        loop {
            if !event::poll(Duration::from_millis(50))? {
                on_idle()?;
                continue;
            }

            let event = event::read()?;
            if let Event::Key(key) = &event {
                // Windows terminals may report both press and release events.
                // Only a press should mutate the input buffer.
                if key.kind != KeyEventKind::Press {
                    continue;
                }
            }

            match event {
                Event::Key(KeyEvent {
                    code: KeyCode::Char(c),
                    modifiers,
                    ..
                }) if !modifiers.contains(KeyModifiers::CONTROL) => {
                    input.push(c);

                    // Check if should show dropdown
                    if input.starts_with('/') {
                        dropdown_visible = true;
                        suggestions = self.get_completions(&input);
                        selected_index = 0;
                    } else {
                        dropdown_visible = false;
                    }

                    // Re-render line and dropdown
                    self.render_line_and_dropdown(
                        &input,
                        &suggestions,
                        selected_index,
                        dropdown_visible,
                    )?;
                }

                Event::Key(KeyEvent {
                    code: KeyCode::Backspace,
                    ..
                }) => {
                    input.pop();
                    dropdown_visible = input.starts_with('/');

                    if dropdown_visible {
                        suggestions = self.get_completions(&input);
                        selected_index = 0;
                    }

                    self.render_line_and_dropdown(
                        &input,
                        &suggestions,
                        selected_index,
                        dropdown_visible,
                    )?;
                }

                Event::Key(KeyEvent {
                    code: KeyCode::Up, ..
                }) if dropdown_visible => {
                    if !suggestions.is_empty() {
                        selected_index = selected_index.saturating_sub(1);
                        self.render_line_and_dropdown(&input, &suggestions, selected_index, true)?;
                    }
                }

                Event::Key(KeyEvent {
                    code: KeyCode::Down,
                    ..
                }) if dropdown_visible => {
                    if !suggestions.is_empty() {
                        selected_index =
                            (selected_index + 1).min(suggestions.len().saturating_sub(1));
                        self.render_line_and_dropdown(&input, &suggestions, selected_index, true)?;
                    }
                }

                Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    ..
                }) => {
                    disable_raw_mode()?;
                    if dropdown_visible && !suggestions.is_empty() {
                        // Use selected suggestion
                        let submitted = suggestions[selected_index].clone();
                        self.finish_submitted_line(&submitted)?;
                        return Ok(ReadOutcome::Submit(submitted));
                    }
                    self.finish_submitted_line(&input)?;
                    return Ok(ReadOutcome::Submit(input));
                }

                Event::Key(KeyEvent {
                    code: KeyCode::Esc, ..
                }) => {
                    dropdown_visible = false;
                    self.render_line_and_dropdown(&input, &suggestions, selected_index, false)?;
                }

                Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                }) => {
                    disable_raw_mode()?;
                    return Ok(ReadOutcome::Exit);
                }

                Event::Key(KeyEvent {
                    code: KeyCode::Char('d'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                }) => {
                    disable_raw_mode()?;
                    return Ok(ReadOutcome::Exit);
                }

                _ => {
                    // Ignore other keys
                }
            }
        }
    }

    fn get_completions(&self, input: &str) -> Vec<String> {
        if let Some(helper) = self.editor.helper() {
            let context = CompletionContext::new(input.to_string(), input.len());
            let suggestions = helper.completion_engine.get_completions(&context);
            suggestions.into_iter().map(|s| s.display).collect()
        } else {
            Vec::new()
        }
    }

    fn render_line_and_dropdown(
        &mut self,
        input: &str,
        suggestions: &[String],
        selected: usize,
        show_dropdown: bool,
    ) -> io::Result<()> {
        use crossterm::cursor;
        use crossterm::style;
        use crossterm::terminal;
        use crossterm::QueueableCommand;
        use std::io::Write;

        let mut stdout = io::stdout();

        if self.composer_visible {
            crossterm::queue!(stdout, style::Print("\x1b[1A\r"))?;
        }

        // Return to the start of the editable line before clearing and
        // repainting. Clearing from the previous cursor position leaves the
        // old prompt in place and duplicates the prompt on every keystroke.
        crossterm::queue!(
            stdout,
            cursor::MoveToColumn(0),
            terminal::Clear(terminal::ClearType::CurrentLine),
            terminal::Clear(terminal::ClearType::FromCursorDown)
        )?;

        let width = terminal::size().map_or(80, |(width, _)| width.max(20)) as usize;
        let line = format!("{}{input}", self.prompt);
        let padding = " ".repeat(width.saturating_sub(line.chars().count()));
        let empty_row = " ".repeat(width);
        crossterm::queue!(
            stdout,
            style::Print("\x1b[48;5;238m\x1b[97m"),
            style::Print(&empty_row),
            style::Print("\x1b[0m\r\n"),
            style::Print("\x1b[48;5;238m\x1b[97m"),
            style::Print(&line),
            style::Print(padding),
            style::Print("\x1b[0m\r\n"),
            style::Print("\x1b[48;5;238m"),
            style::Print(&empty_row),
            style::Print("\x1b[0m")
        )?;

        let mut rows_below = 1;
        if !self.footer.is_empty() {
            let footer: String = self.footer.chars().take(width).collect();
            crossterm::queue!(
                stdout,
                style::Print("\r\n\x1b[90m"),
                style::Print(footer),
                style::Print("\x1b[0m")
            )?;
            rows_below += 1;
        }

        if show_dropdown && !suggestions.is_empty() {
            // Move below the footer before rendering suggestions.
            crossterm::queue!(stdout, style::Print("\r\n"))?;
            rows_below += 1;

            // Render simple list
            for (i, suggestion) in suggestions.iter().enumerate() {
                if i == selected {
                    // Highlight selected with blue background
                    crossterm::queue!(stdout, style::Print("  "))?;
                    crossterm::queue!(stdout, style::Print(suggestion.clone()))?;
                    crossterm::queue!(stdout, style::Print("\n"))?;
                } else {
                    crossterm::queue!(stdout, style::Print("  "))?;
                    crossterm::queue!(stdout, style::Print(suggestion))?;
                    crossterm::queue!(stdout, style::Print("\n"))?;
                }
                rows_below += 1;
            }
        }

        // Move cursor back to the end of the editable prompt. The prompt is
        // intentionally plain; using its actual width avoids the
        // repeated-character corruption caused by the old hard-coded column.
        let prompt_width = self.prompt.chars().count() as u16;
        if rows_below > 0 {
            crossterm::queue!(stdout, style::Print(format!("\x1b[{rows_below}A\r")))?;
        }
        crossterm::queue!(
            stdout,
            cursor::MoveToColumn(prompt_width.saturating_add(input.len() as u16))
        )?;
        stdout.flush()?;
        self.composer_visible = true;

        Ok(())
    }

    fn finish_submitted_line(&mut self, _input: &str) -> io::Result<()> {
        use crossterm::cursor;
        use crossterm::style;
        use crossterm::terminal;
        use crossterm::QueueableCommand;

        let mut stdout = io::stdout();
        if self.composer_visible {
            crossterm::queue!(stdout, style::Print("\x1b[1A\r"))?;
        }
        crossterm::queue!(
            stdout,
            cursor::MoveToColumn(0),
            terminal::Clear(terminal::ClearType::FromCursorDown)
        )?;
        stdout.flush()?;
        self.composer_visible = false;
        Ok(())
    }

    pub fn read_line(&mut self) -> io::Result<ReadOutcome> {
        if !io::stdin().is_terminal() || !io::stdout().is_terminal() {
            return self.read_line_fallback();
        }

        if let Some(helper) = self.editor.helper_mut() {
            helper.reset_current_line();
        }

        match self.editor.readline(&self.prompt) {
            Ok(line) => Ok(ReadOutcome::Submit(line)),
            Err(ReadlineError::Interrupted) => {
                let has_input = !self.current_line().is_empty();
                self.finish_interrupted_read()?;
                if has_input {
                    Ok(ReadOutcome::Cancel)
                } else {
                    Ok(ReadOutcome::Exit)
                }
            }
            Err(ReadlineError::Eof) => {
                self.finish_interrupted_read()?;
                Ok(ReadOutcome::Exit)
            }
            Err(error) => Err(io::Error::other(error)),
        }
    }

    fn current_line(&self) -> String {
        self.editor
            .helper()
            .map_or_else(String::new, SlashCommandHelper::current_line)
    }

    fn finish_interrupted_read(&mut self) -> io::Result<()> {
        if let Some(helper) = self.editor.helper_mut() {
            helper.reset_current_line();
        }
        let mut stdout = io::stdout();
        writeln!(stdout)
    }

    fn read_line_fallback(&self) -> io::Result<ReadOutcome> {
        let mut stdout = io::stdout();
        let prompt = self.prompt.replace(['\u{1}', '\u{2}'], "");
        write!(stdout, "{prompt}")?;
        stdout.flush()?;

        let mut buffer = String::new();
        let bytes_read = io::stdin().read_line(&mut buffer)?;
        if bytes_read == 0 {
            return Ok(ReadOutcome::Exit);
        }

        while matches!(buffer.chars().last(), Some('\n' | '\r')) {
            buffer.pop();
        }
        Ok(ReadOutcome::Submit(buffer))
    }
}

fn slash_command_prefix(line: &str, pos: usize) -> Option<&str> {
    if pos != line.len() {
        return None;
    }

    let prefix = &line[..pos];
    if !prefix.starts_with('/') {
        return None;
    }

    Some(prefix)
}

fn normalize_completions(completions: Vec<String>) -> Vec<String> {
    let mut seen = BTreeSet::new();
    completions
        .into_iter()
        .filter(|candidate| candidate.starts_with('/'))
        .filter(|candidate| seen.insert(candidate.clone()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{slash_command_prefix, LineEditor, SlashCommandHelper};
    use rustyline::completion::Completer;
    use rustyline::highlight::Highlighter;
    use rustyline::history::{DefaultHistory, History};
    use rustyline::Context;

    #[test]
    fn extracts_terminal_slash_command_prefixes_with_arguments() {
        assert_eq!(slash_command_prefix("/he", 3), Some("/he"));
        assert_eq!(slash_command_prefix("/help me", 8), Some("/help me"));
        assert_eq!(
            slash_command_prefix("/session switch ses", 19),
            Some("/session switch ses")
        );
        assert_eq!(slash_command_prefix("hello", 5), None);
        assert_eq!(slash_command_prefix("/help", 2), None);
    }

    #[test]
    fn completes_matching_slash_commands() {
        let helper = SlashCommandHelper::new(vec![
            "/help".to_string(),
            "/hello".to_string(),
            "/status".to_string(),
        ]);
        let history = DefaultHistory::new();
        let ctx = Context::new(&history);
        let (start, matches) = helper
            .complete("/he", 3, &ctx)
            .expect("completion should work");

        assert_eq!(start, 0);
        assert_eq!(
            matches
                .into_iter()
                .map(|candidate| candidate.replacement)
                .collect::<Vec<_>>(),
            vec!["/help ".to_string()]
        );
    }

    #[test]
    fn completes_matching_slash_command_arguments() {
        let helper = SlashCommandHelper::new(vec![
            "/model".to_string(),
            "/model opus".to_string(),
            "/model sonnet".to_string(),
            "/session switch alpha".to_string(),
        ]);
        let history = DefaultHistory::new();
        let ctx = Context::new(&history);
        let (start, matches) = helper
            .complete("/model o", 8, &ctx)
            .expect("completion should work");

        assert_eq!(start, 0);
        assert_eq!(
            matches
                .into_iter()
                .map(|candidate| candidate.replacement)
                .collect::<Vec<_>>(),
            vec!["opus ".to_string()]
        );
    }

    #[test]
    fn ignores_non_slash_command_completion_requests() {
        let helper = SlashCommandHelper::new(vec!["/help".to_string()]);
        let history = DefaultHistory::new();
        let ctx = Context::new(&history);
        let (_, matches) = helper
            .complete("hello", 5, &ctx)
            .expect("completion should work");

        assert!(matches.is_empty());
    }

    #[test]
    fn tracks_current_buffer_through_highlighter() {
        let helper = SlashCommandHelper::new(Vec::new());
        let _ = helper.highlight("draft", 5);

        assert_eq!(helper.current_line(), "draft");
    }

    #[test]
    fn push_history_ignores_blank_entries() {
        let mut editor = LineEditor::new("> ", vec!["/help".to_string()]);
        editor.push_history("   ");
        editor.push_history("/help");

        assert_eq!(editor.editor.history().len(), 1);
    }

    #[test]
    fn set_completions_replaces_and_normalizes_candidates() {
        let mut editor = LineEditor::new("> ", vec!["/help".to_string()]);
        editor.set_completions(vec![
            "/model opus".to_string(),
            "/model opus".to_string(),
            "status".to_string(),
        ]);

        let helper = editor.editor.helper().expect("helper should exist");
        assert_eq!(helper.completions, vec!["/model opus".to_string()]);
    }
}
