use std::io;

fn main() {
    println!("🔄 Palindrome checker");
    println!("Enter a string to check if it is a palindrome:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("❌ Failed to read input");

    let cleaned_input = clean_input(&input);
    if cleaned_input.is_empty() {
        println!("❌ Please enter a valid non-empty string");

        return;
    }

    if is_palindrome(&cleaned_input) {
        println!("✅ '{}' is a palindrome!", input.trim())
    } else {
        println!("❌ '{}' is not a palindrome!", input.trim())
    }
}

fn clean_input(value: &str) -> String {
    value
        .trim()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_lowercase().to_string())
        .collect::<String>()
}

fn is_palindrome(s: &str) -> bool {
    s == s.chars().rev().collect::<String>()
}
