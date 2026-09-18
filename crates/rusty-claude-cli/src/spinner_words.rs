/// Cave-themed words for the Oog Code loading animation.
/// These are displayed during AI thinking states with a shimmering gradient effect

pub static LOADING_WORDS: &[&str] = &[
    // Christianity/Jesus themed
    "Grace",
    "Amen",
    "Faith",
    "Hope",
    "Love",
    "Peace",
    "Joy",
    "Light",
    "Truth",
    "Spirit",
    "Prayer",
    "Blessed",
    "Glory",
    "Hallelujah",
    "Salvation",
    "Redeemed",
    "Sanctuary",
    "Reverence",
    "Devotion",
    "Providence",
    // Troglodytic branding
    "Troglodytic",
    "Cavern",
    "Oog",
    "Sincere",
    "Insight",
    "Sunrise",
    "Mindful",
    "Miracle",
    "Mystery",
    "Harmony",
    // Additional meaningful words
    "Wisdom",
    "Patience",
    "Kindness",
    "Gentleness",
    "Faithfulness",
    "Goodness",
    "Justice",
    "Mercy",
    "Compassion",
    "Understanding",
];

/// Select a random word based on timestamp
///
/// Uses the current timestamp (in seconds) to select a word,
/// ensuring the same word is used throughout a single loading session
pub fn random_loading_word() -> String {
    use std::time::SystemTime;

    // Use seconds since epoch for consistent word per session
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let index = (timestamp as usize) % LOADING_WORDS.len();
    LOADING_WORDS[index].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loading_words_not_empty() {
        assert!(!LOADING_WORDS.is_empty());
        assert!(LOADING_WORDS.len() > 30);
    }

    #[test]
    fn test_all_words_are_alpha() {
        for word in LOADING_WORDS {
            assert!(
                word.chars().all(char::is_alphabetic),
                "Word '{word}' should be alphabetic"
            );
        }
    }

    #[test]
    fn test_random_loading_word_returns_valid_word() {
        let word = random_loading_word();
        assert!(LOADING_WORDS.contains(&word.as_str()));
    }

    #[test]
    fn test_random_loading_word_is_string() {
        let word = random_loading_word();
        assert!(!word.is_empty());
    }
}
