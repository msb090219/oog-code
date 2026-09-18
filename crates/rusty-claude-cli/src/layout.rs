use crossterm::terminal;
use std::io::{self, Result};

/// Calculate the content width based on terminal size
/// Returns 75% of terminal width, max 100 characters
pub fn calculate_content_width() -> usize {
    match terminal::size() {
        Ok((width, _)) => {
            let width_75_percent = (f32::from(width) * 0.75) as usize;
            width_75_percent.clamp(60, 100)
        }
        Err(_) => 80, // Fallback to 80 chars if we can't get terminal size
    }
}

/// Draw the top of a box
pub fn draw_box_top(width: usize) -> String {
    let horizontal = "─".repeat(width.saturating_sub(2));
    format!("╭{horizontal}╮\n")
}

/// Draw a middle line of a box (for inner content separation)
pub fn draw_box_middle(width: usize) -> String {
    let horizontal = "─".repeat(width.saturating_sub(2));
    format!("│{horizontal}│\n")
}

/// Draw the bottom of a box
pub fn draw_box_bottom(width: usize) -> String {
    let horizontal = "─".repeat(width.saturating_sub(2));
    format!("╰{horizontal}╯\n")
}

/// Center text within a given width
pub fn center_text(text: &str, width: usize) -> String {
    let text_len = text.chars().count();
    if text_len >= width {
        return text.chars().take(width).collect();
    }

    let padding = (width - text_len) / 2;
    let left_padding = " ".repeat(padding);
    let right_padding = " ".repeat(width - text_len - padding);

    format!("{left_padding}{text}{right_padding}")
}

/// Wrap text to fit within a specified width
pub fn wrap_text(text: &str, width: usize) -> String {
    let mut result = String::new();
    let mut current_line = String::new();
    let mut current_length = 0;

    for word in text.split_whitespace() {
        let word_len = word.chars().count();

        if current_length == 0 {
            // First word on line
            current_line = word.to_string();
            current_length = word_len;
        } else if current_length + 1 + word_len <= width {
            // Word fits on current line
            current_line.push(' ');
            current_line.push_str(word);
            current_length += 1 + word_len;
        } else {
            // Word doesn't fit, start new line
            result.push_str(&current_line);
            result.push('\n');
            current_line = word.to_string();
            current_length = word_len;
        }
    }

    // Add the last line
    if !current_line.is_empty() {
        result.push_str(&current_line);
    }

    result
}

/// Wrap text while preserving ANSI escape sequences
/// This wraps at word boundaries and tracks visible character width
/// while preserving ANSI color codes and other escape sequences
pub fn wrap_text_ansi_words(text: &str, width: usize) -> String {
    let mut result = String::new();
    let mut current_line = String::new();
    let mut current_visible_length = 0;

    for word in text.split_whitespace() {
        let word_visible_length = strip_ansi_simple(word).chars().count();

        if current_visible_length == 0 {
            current_line = word.to_string();
            current_visible_length = word_visible_length;
        } else if current_visible_length + 1 + word_visible_length <= width {
            current_line.push(' ');
            current_line.push_str(word);
            current_visible_length += 1 + word_visible_length;
        } else {
            result.push_str(&current_line);
            result.push('\n');
            current_line = word.to_string();
            current_visible_length = word_visible_length;
        }
    }

    if !current_line.is_empty() {
        result.push_str(&current_line);
    }

    result
}

/// Simple ANSI stripper for width calculation
/// Removes ANSI escape sequences to calculate visible character width
fn strip_ansi_simple(input: &str) -> String {
    let mut output = String::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\u{1b}' {
            // Start of ANSI escape sequence
            if chars.peek() == Some(&'[') {
                chars.next(); // Consume the '['
                              // Consume until we hit a alphabetic character (end of sequence)
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

/// Draw a horizontal divider line
pub fn draw_divider(width: usize, color: Option<&str>) -> String {
    let line = "─".repeat(width);
    if let Some(color_code) = color {
        format!("{}\x1b[0m\n", color_code.to_string() + &line)
    } else {
        format!("{line}\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_content_width() {
        let width = calculate_content_width();
        assert!(width >= 60);
        assert!(width <= 100);
    }

    #[test]
    fn test_center_text() {
        assert_eq!(center_text("test", 10), "   test   ");
        assert_eq!(center_text("hello", 8), " hello  ");
    }

    #[test]
    fn test_wrap_text() {
        let wrapped = wrap_text("hello world this is a test", 10);
        assert!(wrapped.contains('\n'));
        let lines: Vec<&str> = wrapped.lines().collect();
        assert!(lines.len() > 1);
    }

    #[test]
    fn test_draw_box_top() {
        let top = draw_box_top(10);
        assert!(top.starts_with('╭'));
        assert!(top.ends_with("╮\n"));
    }

    #[test]
    fn test_draw_box_bottom() {
        let bottom = draw_box_bottom(10);
        assert!(bottom.starts_with('╰'));
        assert!(bottom.ends_with("╯\n"));
    }

    #[test]
    fn test_wrap_text_ansi_preserves_colors() {
        let text = "\x1b[31mred text\x1b[0m and more";
        let wrapped = wrap_text_ansi_words(text, 20);
        assert!(wrapped.contains("\x1b[31m"));
        assert!(wrapped.contains("\x1b[0m"));
    }

    #[test]
    fn test_wrap_text_ansi_words() {
        let text = "hello world this is a test";
        let wrapped = wrap_text_ansi_words(text, 10);
        let lines: Vec<&str> = wrapped.lines().collect();
        assert!(lines.len() > 1);
    }

    #[test]
    fn test_strip_ansi_simple() {
        let colored = "\x1b[31mred\x1b[0m";
        let stripped = strip_ansi_simple(colored);
        assert_eq!(stripped, "red");
    }
}
