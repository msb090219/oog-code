use std::env;
use std::fmt::Write as FmtWrite;
use std::io::{self, Write};

use crate::theme::{SemanticColors, Spacing, Theme as UiTheme, ThemePreset};

use crossterm::cursor::{MoveToColumn, RestorePosition, SavePosition};
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor, Stylize};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{execute, queue};
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag, TagEnd};
use syntect::easy::HighlightLines;
use syntect::highlighting::{Theme, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

/// Color theme using semantic tokens from the theme system
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorTheme {
    /// Heading color (bone)
    pub heading: Color,
    /// Emphasis color (muted)
    pub emphasis: Color,
    /// Strong/bold color
    pub strong: Color,
    /// Inline code color (green)
    pub inline_code: Color,
    /// Link color (ochre, underlined)
    pub link: Color,
    /// Quote/blockquote color (muted)
    pub quote: Color,
    /// Table border color (subtle)
    pub table_border: Color,
    /// Spinner active color (muted gray)
    pub spinner_active: Color,
    /// Spinner done color (success green)
    pub spinner_done: Color,
    /// Spinner failed color (error red)
    pub spinner_failed: Color,
}

impl ColorTheme {
    /// Create `ColorTheme` from Theme system
    pub fn from_theme(theme: &UiTheme) -> Self {
        let semantic = &theme.semantic;

        Self {
            heading: semantic.secondary,
            // Emphasis: muted gray for secondary text
            emphasis: semantic.muted,
            // Strong: bold, keep terminal default
            strong: Color::Reset,
            // Inline code: success green
            inline_code: semantic.success,
            link: semantic.primary,
            // Quote: muted gray
            quote: semantic.muted,
            // Table border: subtle gray
            table_border: semantic.muted,
            // Spinner colors
            spinner_active: semantic.muted,
            spinner_done: semantic.success,
            spinner_failed: semantic.error,
        }
    }

    /// Get spacing configuration
    pub fn spacing(&self) -> Spacing {
        Spacing::default()
    }
}

impl Default for ColorTheme {
    fn default() -> Self {
        // Create default theme and convert
        let theme = UiTheme::new(ThemePreset::Default);
        Self::from_theme(&theme)
    }
}

/// Simple spinner with dot animation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spinner {
    frame_index: usize,
}

impl Spinner {
    const FRAMES: [&str; 4] = ["⠁", "⠂", "⠃", "⠄"];

    #[must_use]
    pub fn new() -> Self {
        Self { frame_index: 0 }
    }

    pub fn tick(
        &mut self,
        label: &str,
        theme: &ColorTheme,
        out: &mut impl Write,
    ) -> io::Result<()> {
        let frame = Self::FRAMES[self.frame_index % Self::FRAMES.len()];
        queue!(
            out,
            SavePosition,
            MoveToColumn(0),
            Clear(ClearType::CurrentLine),
            SetForegroundColor(theme.spinner_active),
            Print(format!("{frame} {label}")),
            ResetColor,
            RestorePosition
        )?;
        self.frame_index += 1;
        out.flush()
    }

    pub fn finish(
        &mut self,
        label: &str,
        theme: &ColorTheme,
        out: &mut impl Write,
    ) -> io::Result<()> {
        self.frame_index = 0;
        execute!(
            out,
            MoveToColumn(0),
            Clear(ClearType::CurrentLine),
            SetForegroundColor(theme.spinner_done),
            Print(format!("✔ {label}\n")),
            ResetColor
        )?;
        out.flush()
    }

    pub fn fail(
        &mut self,
        label: &str,
        theme: &ColorTheme,
        out: &mut impl Write,
    ) -> io::Result<()> {
        self.frame_index = 0;
        execute!(
            out,
            MoveToColumn(0),
            Clear(ClearType::CurrentLine),
            SetForegroundColor(theme.spinner_failed),
            Print(format!("✘ {label}\n")),
            ResetColor
        )?;
        out.flush()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ListKind {
    Unordered,
    Ordered { next_index: u64 },
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct TableState {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    current_row: Vec<String>,
    current_cell: String,
    in_head: bool,
}

impl TableState {
    fn push_cell(&mut self) {
        let cell = self.current_cell.trim().to_string();
        self.current_row.push(cell);
        self.current_cell.clear();
    }

    fn finish_row(&mut self) {
        if self.current_row.is_empty() {
            return;
        }
        let row = std::mem::take(&mut self.current_row);
        if self.in_head {
            self.headers = row;
        } else {
            self.rows.push(row);
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct RenderState {
    emphasis: usize,
    strong: usize,
    heading_level: Option<u8>,
    quote: usize,
    list_stack: Vec<ListKind>,
    link_stack: Vec<LinkState>,
    table: Option<TableState>,
    previous_block_type: Option<BlockType>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LinkState {
    destination: String,
    text: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BlockType {
    Paragraph,
    Heading,
    List,
    CodeBlock,
    Table,
    BlockQuote,
}

impl RenderState {
    fn style_text(&self, text: &str, theme: &ColorTheme) -> String {
        let mut style = text.stylize();

        if matches!(self.heading_level, Some(1 | 2)) || self.strong > 0 {
            style = style.bold();
        }
        if self.emphasis > 0 {
            style = style.italic();
        }

        if let Some(level) = self.heading_level {
            style = match level {
                1 => style.with(theme.heading),
                2 => style.white(),
                3 => style.with(Color::Rgb {
                    r: 100,
                    g: 180,
                    b: 255,
                }),
                _ => style.with(Color::Grey),
            };
        } else if self.strong > 0 {
            style = style.with(theme.strong);
        } else if self.emphasis > 0 {
            style = style.with(theme.emphasis);
        }

        if self.quote > 0 {
            style = style.with(theme.quote);
        }

        format!("{style}")
    }

    fn append_raw(&mut self, output: &mut String, text: &str) {
        if let Some(link) = self.link_stack.last_mut() {
            link.text.push_str(text);
        } else if let Some(table) = self.table.as_mut() {
            table.current_cell.push_str(text);
        } else {
            output.push_str(text);
        }
    }

    fn append_styled(&mut self, output: &mut String, text: &str, theme: &ColorTheme) {
        let styled = self.style_text(text, theme);
        self.append_raw(output, &styled);
    }

    fn should_insert_divider(
        &self,
        current_block_type: BlockType,
        heading_level: Option<u8>,
    ) -> bool {
        match self.previous_block_type {
            None => false,
            Some(previous) => {
                // Never around code blocks
                if matches!(previous, BlockType::CodeBlock)
                    || matches!(current_block_type, BlockType::CodeBlock)
                {
                    return false;
                }
                // Only before H1/H2 headings
                if matches!(current_block_type, BlockType::Heading) {
                    return matches!(heading_level, Some(1 | 2));
                }
                // No dividers elsewhere
                false
            }
        }
    }

    fn update_block_type(&mut self, block_type: BlockType) {
        self.previous_block_type = Some(block_type);
    }
}

#[derive(Debug)]
pub struct TerminalRenderer {
    syntax_set: SyntaxSet,
    syntax_theme: Theme,
    color_theme: ColorTheme,
}

impl Default for TerminalRenderer {
    fn default() -> Self {
        let syntax_set = SyntaxSet::load_defaults_newlines();
        let syntax_theme = ThemeSet::load_defaults()
            .themes
            .remove("base16-ocean.dark")
            .unwrap_or_default();
        Self {
            syntax_set,
            syntax_theme,
            color_theme: ColorTheme::default(),
        }
    }
}

impl TerminalRenderer {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn color_theme(&self) -> &ColorTheme {
        &self.color_theme
    }

    fn render_divider(&self, output: &mut String) {
        let divider = "─".repeat(80);
        let styled = format!("{}", divider.with(self.color_theme.emphasis));
        output.push_str(&styled);
        output.push('\n');
    }

    #[must_use]
    pub fn render_markdown(&self, markdown: &str) -> String {
        let mut output = String::new();
        let mut state = RenderState::default();
        let mut code_language = String::new();
        let mut code_buffer = String::new();
        let mut in_code_block = false;

        for event in Parser::new_ext(markdown, Options::all()) {
            self.render_event(
                event,
                &mut state,
                &mut output,
                &mut code_buffer,
                &mut code_language,
                &mut in_code_block,
            );
        }

        output.trim_end().to_string()
    }

    #[must_use]
    pub fn markdown_to_ansi(&self, markdown: &str) -> String {
        self.render_markdown(markdown)
    }

    #[allow(clippy::too_many_lines)]
    fn render_event(
        &self,
        event: Event<'_>,
        state: &mut RenderState,
        output: &mut String,
        code_buffer: &mut String,
        code_language: &mut String,
        in_code_block: &mut bool,
    ) {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                if state.should_insert_divider(BlockType::Heading, Some(level as u8)) {
                    self.render_divider(output);
                }
                Self::start_heading(state, level as u8, output);
                state.update_block_type(BlockType::Heading);
            }
            Event::End(TagEnd::Paragraph) => {
                state.update_block_type(BlockType::Paragraph);
                output.push_str("\n\n");
            }
            Event::Start(Tag::BlockQuote(..)) => {
                if state.should_insert_divider(BlockType::BlockQuote, None) {
                    self.render_divider(output);
                }
                self.start_quote(state, output);
            }
            Event::End(TagEnd::BlockQuote(..)) => {
                state.quote = state.quote.saturating_sub(1);
                state.update_block_type(BlockType::BlockQuote);
                output.push('\n');
            }
            Event::End(TagEnd::Heading(..)) => {
                state.heading_level = None;
                output.push_str("\n\n");
            }
            Event::End(TagEnd::Item) | Event::SoftBreak | Event::HardBreak => {
                state.append_raw(output, "\n");
            }
            Event::Start(Tag::List(first_item)) => {
                if state.should_insert_divider(BlockType::List, None) {
                    self.render_divider(output);
                }
                let kind = match first_item {
                    Some(index) => ListKind::Ordered { next_index: index },
                    None => ListKind::Unordered,
                };
                state.list_stack.push(kind);
            }
            Event::End(TagEnd::List(..)) => {
                state.list_stack.pop();
                state.update_block_type(BlockType::List);
                output.push('\n');
            }
            Event::Start(Tag::Item) => Self::start_item(state, output),
            Event::Start(Tag::CodeBlock(kind)) => {
                state.update_block_type(BlockType::CodeBlock);
                *in_code_block = true;
                *code_language = match kind {
                    CodeBlockKind::Indented => String::from("text"),
                    CodeBlockKind::Fenced(lang) => lang.to_string(),
                };
                code_buffer.clear();
                self.start_code_block(code_language, output);
            }
            Event::End(TagEnd::CodeBlock) => {
                self.finish_code_block(code_buffer, code_language, output);
                *in_code_block = false;
                code_language.clear();
                code_buffer.clear();
            }
            Event::Start(Tag::Emphasis) => state.emphasis += 1,
            Event::End(TagEnd::Emphasis) => state.emphasis = state.emphasis.saturating_sub(1),
            Event::Start(Tag::Strong) => state.strong += 1,
            Event::End(TagEnd::Strong) => state.strong = state.strong.saturating_sub(1),
            Event::Code(code) => {
                let rendered =
                    format!("{}", format!("`{code}`").with(self.color_theme.inline_code));
                state.append_raw(output, &rendered);
            }
            Event::Rule => output.push_str("---\n"),
            Event::Text(text) => {
                self.push_text(text.as_ref(), state, output, code_buffer, *in_code_block);
            }
            Event::Html(html) | Event::InlineHtml(html) => {
                state.append_raw(output, &html);
            }
            Event::FootnoteReference(reference) => {
                state.append_raw(output, &format!("[{reference}]"));
            }
            Event::TaskListMarker(done) => {
                state.append_raw(output, if done { "[x] " } else { "[ ] " });
            }
            Event::InlineMath(math) | Event::DisplayMath(math) => {
                state.append_raw(output, &math);
            }
            Event::Start(Tag::Link { dest_url, .. }) => {
                state.link_stack.push(LinkState {
                    destination: dest_url.to_string(),
                    text: String::new(),
                });
            }
            Event::End(TagEnd::Link) => {
                if let Some(link) = state.link_stack.pop() {
                    let label = if link.text.is_empty() {
                        link.destination.clone()
                    } else {
                        link.text
                    };
                    let rendered = format!(
                        "{}",
                        format!("[{label}]({})", link.destination)
                            .underlined()
                            .with(self.color_theme.link)
                    );
                    state.append_raw(output, &rendered);
                }
            }
            Event::Start(Tag::Image { dest_url, .. }) => {
                let rendered = format!(
                    "{}",
                    format!("[image:{dest_url}]").with(self.color_theme.link)
                );
                state.append_raw(output, &rendered);
            }
            Event::Start(Tag::Table(..)) => {
                if state.should_insert_divider(BlockType::Table, None) {
                    self.render_divider(output);
                }
                state.table = Some(TableState::default());
            }
            Event::End(TagEnd::Table) => {
                if let Some(table) = state.table.take() {
                    output.push_str(&self.render_table(&table));
                    output.push_str("\n\n");
                }
                state.update_block_type(BlockType::Table);
            }
            Event::Start(Tag::TableHead) => {
                if let Some(table) = state.table.as_mut() {
                    table.in_head = true;
                }
            }
            Event::End(TagEnd::TableHead) => {
                if let Some(table) = state.table.as_mut() {
                    table.finish_row();
                    table.in_head = false;
                }
            }
            Event::Start(Tag::TableRow) => {
                if let Some(table) = state.table.as_mut() {
                    table.current_row.clear();
                    table.current_cell.clear();
                }
            }
            Event::End(TagEnd::TableRow) => {
                if let Some(table) = state.table.as_mut() {
                    table.finish_row();
                }
            }
            Event::Start(Tag::TableCell) => {
                if let Some(table) = state.table.as_mut() {
                    table.current_cell.clear();
                }
            }
            Event::End(TagEnd::TableCell) => {
                if let Some(table) = state.table.as_mut() {
                    table.push_cell();
                }
            }
            Event::Start(Tag::Paragraph | Tag::MetadataBlock(..) | _)
            | Event::End(TagEnd::Image | TagEnd::MetadataBlock(..) | _) => {}
        }
    }

    fn start_heading(state: &mut RenderState, level: u8, output: &mut String) {
        state.heading_level = Some(level);
        if !output.is_empty() && !output.ends_with("\n\n") {
            output.push('\n');
        }
    }

    fn start_quote(&self, state: &mut RenderState, output: &mut String) {
        state.quote += 1;
        let _ = write!(output, "{}", "│ ".with(self.color_theme.quote));
    }

    fn start_item(state: &mut RenderState, output: &mut String) {
        let depth = state.list_stack.len().saturating_sub(1);
        output.push_str(&"  ".repeat(depth));

        let marker = match state.list_stack.last_mut() {
            Some(ListKind::Ordered { next_index }) => {
                let value = *next_index;
                *next_index += 1;
                format!("{value}. ")
            }
            _ => "• ".to_string(),
        };
        output.push_str(&marker);
    }

    fn start_code_block(&self, code_language: &str, output: &mut String) {
        let label = if code_language.is_empty() {
            "text"
        } else {
            code_language
        };
        if !output.is_empty() && !output.ends_with('\n') {
            output.push('\n');
        }
        let _ = writeln!(output, "\x1b[48;5;238;38;5;255m {label} \x1b[0m");
    }

    fn finish_code_block(&self, code_buffer: &str, code_language: &str, output: &mut String) {
        output.push_str(&self.highlight_code(code_buffer, code_language));
        output.push_str("\x1b[48;5;238m \x1b[0m");
        output.push_str("\n\n");
    }

    fn push_text(
        &self,
        text: &str,
        state: &mut RenderState,
        output: &mut String,
        code_buffer: &mut String,
        in_code_block: bool,
    ) {
        if in_code_block {
            code_buffer.push_str(text);
        } else {
            state.append_styled(output, text, &self.color_theme);
        }
    }

    fn render_table(&self, table: &TableState) -> String {
        let mut rows = Vec::new();
        if !table.headers.is_empty() {
            rows.push(table.headers.clone());
        }
        rows.extend(table.rows.iter().cloned());

        if rows.is_empty() {
            return String::new();
        }

        let column_count = rows.iter().map(Vec::len).max().unwrap_or(0);
        let widths = (0..column_count)
            .map(|column| {
                rows.iter()
                    .filter_map(|row| row.get(column))
                    .map(|cell| visible_width(cell))
                    .max()
                    .unwrap_or(0)
            })
            .collect::<Vec<_>>();

        let border = format!("{}", "│".with(self.color_theme.table_border));
        let separator = widths
            .iter()
            .map(|width| "─".repeat(*width + 2))
            .collect::<Vec<_>>()
            .join(&format!("{}", "┼".with(self.color_theme.table_border)));
        let separator = format!("{border}{separator}{border}");

        let mut output = String::new();
        if !table.headers.is_empty() {
            output.push_str(&self.render_table_row(&table.headers, &widths, true));
            output.push('\n');
            output.push_str(&separator);
            if !table.rows.is_empty() {
                output.push('\n');
            }
        }

        for (index, row) in table.rows.iter().enumerate() {
            output.push_str(&self.render_table_row(row, &widths, false));
            if index + 1 < table.rows.len() {
                output.push('\n');
            }
        }

        output
    }

    fn render_table_row(&self, row: &[String], widths: &[usize], is_header: bool) -> String {
        let border = format!("{}", "│".with(self.color_theme.table_border));
        let mut line = String::new();
        line.push_str(&border);

        for (index, width) in widths.iter().enumerate() {
            let cell = row.get(index).map_or("", String::as_str);
            line.push(' ');
            if is_header {
                let _ = write!(line, "{}", cell.bold().with(self.color_theme.heading));
            } else {
                line.push_str(cell);
            }
            let padding = width.saturating_sub(visible_width(cell));
            line.push_str(&" ".repeat(padding + 1));
            line.push_str(&border);
        }

        line
    }

    #[must_use]
    pub fn highlight_code(&self, code: &str, language: &str) -> String {
        let syntax = self
            .syntax_set
            .find_syntax_by_token(language)
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());
        let mut syntax_highlighter = HighlightLines::new(syntax, &self.syntax_theme);
        let mut colored_output = String::new();

        for line in LinesWithEndings::from(code) {
            match syntax_highlighter.highlight_line(line, &self.syntax_set) {
                Ok(ranges) => {
                    let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
                    colored_output.push_str(&apply_code_block_background(&escaped));
                }
                Err(_) => colored_output.push_str(&apply_code_block_background(line)),
            }
        }

        colored_output
    }

    pub fn stream_markdown(
        &self,
        markdown: &str,
        out: &mut (impl Write + ?Sized),
    ) -> io::Result<()> {
        let rendered_markdown = self.markdown_to_ansi(markdown);
        write!(out, "{rendered_markdown}")?;
        if !rendered_markdown.ends_with('\n') {
            writeln!(out)?;
        }
        out.flush()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct MarkdownStreamState {
    pending: String,
}

impl MarkdownStreamState {
    #[must_use]
    pub fn push(&mut self, renderer: &TerminalRenderer, delta: &str) -> Option<String> {
        self.pending.push_str(delta);
        let split = find_stream_safe_boundary(&self.pending)?;
        let ready = self.pending[..split].to_string();
        self.pending.drain(..split);
        Some(render_stream_chunk(renderer, &ready))
    }

    #[must_use]
    pub fn flush(&mut self, renderer: &TerminalRenderer) -> Option<String> {
        if self.pending.trim().is_empty() {
            self.pending.clear();
            None
        } else {
            let pending = std::mem::take(&mut self.pending);
            Some(render_stream_chunk(renderer, &pending))
        }
    }
}

fn render_stream_chunk(renderer: &TerminalRenderer, markdown: &str) -> String {
    let trailing_newlines = markdown
        .chars()
        .rev()
        .take_while(|character| *character == '\n')
        .count();
    let mut rendered = renderer.markdown_to_ansi(markdown);
    rendered.push_str(&"\n".repeat(trailing_newlines));
    rendered
}

fn apply_code_block_background(line: &str) -> String {
    let trimmed = line.trim_end_matches('\n');
    let trailing_newline = if trimmed.len() == line.len() {
        ""
    } else {
        "\n"
    };
    let with_background = trimmed.replace("\u{1b}[0m", "\u{1b}[0;48;5;236m");
    format!("\u{1b}[48;5;236m{with_background}\u{1b}[0m{trailing_newline}")
}

fn find_stream_safe_boundary(markdown: &str) -> Option<usize> {
    let mut in_fence = false;
    let mut last_boundary = None;

    for (offset, line) in markdown.split_inclusive('\n').scan(0usize, |cursor, line| {
        let start = *cursor;
        *cursor += line.len();
        Some((start, line))
    }) {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            in_fence = !in_fence;
            if !in_fence {
                last_boundary = Some(offset + line.len());
            }
            continue;
        }

        if in_fence {
            continue;
        }

        if trimmed.is_empty() {
            last_boundary = Some(offset + line.len());
        }
    }

    // If we found a boundary, make sure it doesn't split a word
    if let Some(boundary) = last_boundary {
        // Check if the boundary is in the middle of a word
        let before_boundary = &markdown[..boundary];
        let after_boundary = &markdown[boundary..];

        // If before boundary ends with a word character and after starts with one,
        // we're splitting a word - don't do that
        let last_char = before_boundary.chars().last();
        let first_char = after_boundary.chars().next();

        if matches!(last_char, Some(c) if c.is_alphanumeric() || c == '_')
            && matches!(first_char, Some(c) if c.is_alphanumeric() || c == '_')
        {
            // We're splitting a word - look for a safer boundary
            // Find the last whitespace before the boundary
            if let Some(last_space) = before_boundary.rfind(char::is_whitespace) {
                return Some(last_space + 1);
            }
            // No space found, return None to indicate no safe boundary
            return None;
        }

        return Some(boundary);
    }

    last_boundary
}

fn visible_width(input: &str) -> usize {
    strip_ansi(input).chars().count()
}

fn strip_ansi(input: &str) -> String {
    let mut output = String::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\u{1b}' {
            if chars.peek() == Some(&'[') {
                chars.next();
                for next in chars.by_ref() {
                    if next.is_ascii_alphabetic() {
                        break;
                    }
                }
            }
        } else {
            output.push(ch);
        }
    }

    output
}

/// Check if AI responses should have emojis disabled
fn should_disable_emojis() -> bool {
    // Check environment variable MINSEO_DISABLE_EMOJIS
    if env::var("MINSEO_DISABLE_EMOJIS").is_ok() {
        return true;
    }
    false
}

/// Strip emojis from text while preserving other Unicode characters
fn strip_emojis(text: &str) -> String {
    if !should_disable_emojis() {
        return text.to_string();
    }

    // Emoji ranges (Unicode blocks where emojis live)
    // We'll preserve common text symbols but remove decorative emojis
    text.chars()
        .filter(|c| {
            let cp = *c as u32;
            // Keep ASCII and common text symbols
            if cp <= 0x7F {
                return true;
            }
            // Keep common punctuation and symbols (not emojis)
            if (0x2000..=0x206F).contains(&cp) {
                return true; // General punctuation
            }
            // Keep bullet points, arrows, and other text symbols
            if (0x2000..=0x21FF).contains(&cp) && !is_emoji(cp) {
                return true;
            }
            // Keep letters, numbers, and common text
            if (0x0080..=0x00FF).contains(&cp) {
                return true;
            }
            // Keep CJK characters (Chinese, Japanese, Korean)
            if (0x4E00..=0x9FFF).contains(&cp) {
                return true;
            }
            // Filter out emoji ranges
            !is_emoji(cp)
        })
        .collect()
}

/// Check if a code point is an emoji
fn is_emoji(cp: u32) -> bool {
    // Emoji ranges (decorative emojis, not text symbols)
    (0x1F600..=0x1F64F).contains(&cp) || // Emoticons
    (0x1F300..=0x1F5FF).contains(&cp) || // Misc Symbols and Pictographs
    (0x1F680..=0x1F6FF).contains(&cp) || // Transport and Map
    (0x1F900..=0x1F9FF).contains(&cp) || // Supplemental Symbols and Pictographs
    (0x2600..=0x26FF).contains(&cp) ||   // Misc symbols (but we allow some text ones below)
    (0x2700..=0x27BF).contains(&cp) ||   // Dingbats
    (0x1F1E0..=0x1F1FF).contains(&cp) // Flags (regional indicator symbols)
}

#[cfg(test)]
mod tests {
    use super::{strip_ansi, MarkdownStreamState, Spinner, TerminalRenderer};

    #[test]
    fn renders_markdown_with_styling_and_lists() {
        let terminal_renderer = TerminalRenderer::new();
        let markdown_output = terminal_renderer
            .render_markdown("# Heading\n\nThis is **bold** and *italic*.\n\n- item\n\n`code`");

        assert!(markdown_output.contains("Heading"));
        assert!(markdown_output.contains("• item"));
        assert!(markdown_output.contains("code"));
        assert!(markdown_output.contains('\u{1b}'));
    }

    #[test]
    fn headings_do_not_add_leading_blank_lines() {
        let renderer = TerminalRenderer::new();
        let rendered = strip_ansi(&renderer.render_markdown("# Heading\n\nBody"));

        assert!(rendered.starts_with("Heading"));
    }

    #[test]
    fn renders_links_as_colored_markdown_labels() {
        let terminal_renderer = TerminalRenderer::new();
        let markdown_output =
            terminal_renderer.render_markdown("See [Claw](https://example.com/docs) now.");
        let plain_text = strip_ansi(&markdown_output);

        assert!(plain_text.contains("[Claw](https://example.com/docs)"));
        assert!(markdown_output.contains('\u{1b}'));
    }

    #[test]
    fn highlights_fenced_code_blocks() {
        let terminal_renderer = TerminalRenderer::new();
        let markdown_output =
            terminal_renderer.markdown_to_ansi("```rust\nfn hi() { println!(\"hi\"); }\n```");
        let plain_text = strip_ansi(&markdown_output);

        assert!(plain_text.contains("rust"));
        assert!(plain_text.contains("fn hi"));
        assert!(markdown_output.contains('\u{1b}'));
        assert!(markdown_output.contains("[48;5;238m"));
        assert!(markdown_output.contains("[48;5;236m"));
    }

    #[test]
    fn renders_ordered_and_nested_lists() {
        let terminal_renderer = TerminalRenderer::new();
        let markdown_output =
            terminal_renderer.render_markdown("1. first\n2. second\n   - nested\n   - child");
        let plain_text = strip_ansi(&markdown_output);

        assert!(plain_text.contains("1. first"));
        assert!(plain_text.contains("2. second"));
        assert!(plain_text.contains("  • nested"));
        assert!(plain_text.contains("  • child"));
    }

    #[test]
    fn renders_tables_with_alignment() {
        let terminal_renderer = TerminalRenderer::new();
        let markdown_output = terminal_renderer
            .render_markdown("| Name | Value |\n| ---- | ----- |\n| alpha | 1 |\n| beta | 22 |");
        let plain_text = strip_ansi(&markdown_output);
        let lines = plain_text.lines().collect::<Vec<_>>();

        assert_eq!(lines[0], "│ Name  │ Value │");
        assert_eq!(lines[1], "│───────┼───────│");
        assert_eq!(lines[2], "│ alpha │ 1     │");
        assert_eq!(lines[3], "│ beta  │ 22    │");
        assert!(markdown_output.contains('\u{1b}'));
    }

    #[test]
    fn streaming_state_waits_for_complete_blocks() {
        let renderer = TerminalRenderer::new();
        let mut state = MarkdownStreamState::default();

        assert_eq!(state.push(&renderer, "# Heading"), None);
        let flushed = state
            .push(&renderer, "\n\nParagraph\n\n")
            .expect("completed block");
        let plain_text = strip_ansi(&flushed);
        assert!(plain_text.contains("Heading"));
        assert!(plain_text.contains("Paragraph"));

        assert_eq!(state.push(&renderer, "```rust\nfn main() {}\n"), None);
        let code = state
            .push(&renderer, "```\n")
            .expect("closed code fence flushes");
        assert!(strip_ansi(&code).contains("fn main()"));
    }

    #[test]
    fn markdown_stream_preserves_paragraph_breaks() {
        let renderer = TerminalRenderer::new();
        let mut stream = MarkdownStreamState::default();

        let rendered = stream
            .push(&renderer, "first paragraph\n\nsecond paragraph")
            .expect("blank line should produce a safe stream boundary");

        assert!(rendered.contains("first paragraph\n\n"));
        assert!(stream.flush(&renderer).is_some());
    }

    #[test]
    fn spinner_advances_frames() {
        let terminal_renderer = TerminalRenderer::new();
        let mut spinner = Spinner::new();
        let mut out = Vec::new();
        spinner
            .tick("Working", terminal_renderer.color_theme(), &mut out)
            .expect("tick succeeds");
        spinner
            .tick("Working", terminal_renderer.color_theme(), &mut out)
            .expect("tick succeeds");

        let output = String::from_utf8_lossy(&out);
        assert!(output.contains("Working"));
    }
}
