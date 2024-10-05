use std::collections::HashSet;

/// Function to check if all alphabetic characters in the word are unique
pub fn check(word: &str) -> bool {
    let mut seen = HashSet::new();
    word.to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .all(|c| seen.insert(c))
}

fn main() {
    let word1 = "alphabet";
    let word2 = "isogram";
    let word3 = "Rust";

    // Check if word1 has all unique alphabetic characters
    if check(word1) {
        println!("'{}' has all unique alphabetic characters.", word1);
    } else {
        println!("'{}' has repeating alphabetic characters.", word1);
    }

    // Check if word2 has all unique alphabetic characters
    if check(word2) {
        println!("'{}' has all unique alphabetic characters.", word2);
    } else {
        println!("'{}' has repeating alphabetic characters.", word2);
    }

    // Check if word3 has all unique alphabetic characters
    if check(word3) {
        println!("'{}' has all unique alphabetic characters.", word3);
    } else {
        println!("'{}' has repeating alphabetic characters.", word3);
    }
}
