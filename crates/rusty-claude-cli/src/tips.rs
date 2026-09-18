use std::fmt;

/// Random helpful tips for startup
pub static TIPS: &[&str] = &[
    "Use /status to see token usage and cost breakdown.",
    "Press T during any response to expand thinking blocks.",
    "Use /verbosity terse to hide tool output details.",
    "Type /help to see all available commands.",
    "Use /session to switch between conversation sessions.",
    "Press Ctrl+D to exit the REPL.",
    "Use /compact to reduce conversation history size.",
    "Type /diff to see git diff output.",
    "Use /model to switch between AI models.",
    "Press Tab to autocomplete slash commands.",
];

/// Get a random tip
pub fn random_tip() -> &'static str {
    use std::time::{SystemTime, UNIX_EPOCH};

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let index = (timestamp as usize) % TIPS.len();
    TIPS[index]
}

/// Format the startup banner with version and tip
pub fn startup_banner(version: &str) -> String {
    format!("Oog Code v{}\n\nTip: {}\n\n", version, random_tip())
}

/// Get a random tip with "Tip: " prefix for use in the full banner
pub fn random_tip_with_label() -> String {
    format!("Tip: {}", random_tip())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_tip_returns_valid_tip() {
        let tip = random_tip();
        assert!(TIPS.contains(&tip));
    }

    #[test]
    fn test_startup_banner_format() {
        let banner = startup_banner("0.1.0");
        assert!(banner.contains("Oog Code v0.1.0"));
        assert!(banner.contains("Tip:"));
        assert!(banner.contains('\n'));
    }

    #[test]
    fn test_random_tip_with_label() {
        let tip = random_tip_with_label();
        assert!(tip.starts_with("Tip:"));
        assert!(tip.contains("Use") || tip.contains("Press") || tip.contains("Type"));
    }
}
