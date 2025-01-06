use std::io::{self, Write};

fn main() {
    // Print prompt and get input
    print!("Enter the English message to translate into Pig Latin: ");
    io::stdout().flush().unwrap();

    let mut message = String::new();
    io::stdin().read_line(&mut message).unwrap();

    const VOWELS: [char; 6] = ['a', 'e', 'i', 'o', 'u', 'y'];
    let mut pig_latin = Vec::new();

    // Process each word
    for word in message.split_whitespace() {
        let mut current_word = word.to_string();

        // Handle prefix non-letters
        let mut prefix_non_letters = String::new();
        while !current_word.is_empty() && !current_word.chars().next().unwrap().is_alphabetic() {
            prefix_non_letters.push(current_word.chars().next().unwrap());
            current_word = current_word[1..].to_string();
        }

        if current_word.is_empty() {
            pig_latin.push(prefix_non_letters);
            continue;
        }

        // Handle suffix non-letters
        let mut suffix_non_letters = String::new();
        while !current_word.is_empty() && !current_word.chars().last().unwrap().is_alphabetic() {
            suffix_non_letters =
                current_word.chars().last().unwrap().to_string() + &suffix_non_letters;
            current_word = current_word[..current_word.len() - 1].to_string();
        }

        // Remember case
        let was_upper = current_word.chars().all(|c| c.is_uppercase());
        let was_title = current_word.chars().next().unwrap().is_uppercase()
            && current_word[1..].chars().all(|c| c.is_lowercase());

        // Convert to lowercase for processing
        current_word = current_word.to_lowercase();

        // Handle consonants at start
        let mut prefix_consonants = String::new();
        while !current_word.is_empty() && !VOWELS.contains(&current_word.chars().next().unwrap()) {
            prefix_consonants.push(current_word.chars().next().unwrap());
            current_word = current_word[1..].to_string();
        }

        // Add Pig Latin ending
        if prefix_consonants.is_empty() {
            current_word.push_str("yay");
        } else {
            current_word.push_str(&prefix_consonants);
            current_word.push_str("ay");
        }

        // Restore case
        if was_upper {
            current_word = current_word.to_uppercase();
        } else if was_title {
            current_word = current_word[0..1].to_uppercase() + &current_word[1..].to_lowercase();
        }

        // Add back non-letters and push to result
        pig_latin.push(prefix_non_letters + &current_word + &suffix_non_letters);
    }

    // Print result
    println!("{}", pig_latin.join(" "));
}
