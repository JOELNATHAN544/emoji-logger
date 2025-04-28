use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::prelude::IndexedRandom;

const EMOJIS: &[&str] = &["🚀", "🔥", "✨", "💡", "🛠", "🎯", "📦", "🌟"];

/// Logs a message with a random emoji.
pub fn log_with_emoji(message: &str) {
    let emoji = EMOJIS.choose(&mut thread_rng()).unwrap();
    println!("{} {}", emoji, message);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_logs_message_with_emoji() {
        // Just call the function to make sure it doesn't panic
        log_with_emoji("Hello, world!");
    }
}
