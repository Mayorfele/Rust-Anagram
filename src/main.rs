use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};
use csv::ReaderBuilder;

// Hardcoded dictionary path
const DICTIONARY_FOLDER: &str ="C:\\Users\\mayor\\Downloads\\The-English-Open-Word-List-master\\The-English-Open-Word-List-master\\EOWL CSV Format";

fn build_anagram_solver() -> Result<HashMap<String, Vec<String>>, Box<dyn std::error::Error>> {
    let mut anagram_map: HashMap<String, Vec<String>> = HashMap::new();

    for entry in std::fs::read_dir(DICTIONARY_FOLDER)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("csv") {
            let file = File::open(path)?;
            let mut rdr = ReaderBuilder::new()
                .has_headers(false)
                .from_reader(file);

            for result in rdr.records() {
                let record = result?;
                if let Some(word) = record.get(0) {
                    let word = word.trim().to_lowercase();
                    if !word.is_empty() {
                        let sorted_word = sort_word(&word);
                        anagram_map.entry(sorted_word).or_default().push(word);
                    }
                }
            }
        }
    }

    Ok(anagram_map)
}

fn sort_word(word: &str) -> String {
    let mut chars: Vec<char> = word.chars().collect();
    chars.sort_unstable();
    chars.into_iter().collect()
}

fn find_anagrams(word: &str, anagram_map: &HashMap<String, Vec<String>>) -> Vec<String> {
    let sorted_word = sort_word(word);
    anagram_map.get(&sorted_word).cloned().unwrap_or_default()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Building anagram solver from {}...", DICTIONARY_FOLDER);
    let anagram_map = build_anagram_solver()?;
    println!("Welcome to Mayor's Anagram");
    println!("Anagram solver ready!");

    loop {
        println!("\nSelect an option:");
        println!("1. Enter word");
        println!("2. Exit");

        print!("Enter choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "1" => {
                print!("Enter word to find its anagrams: ");
                io::stdout().flush().unwrap();

                let mut word = String::new();
                io::stdin().read_line(&mut word).expect("Failed to read line");

                let word = word.trim().to_lowercase();
                let anagrams = find_anagrams(&word, &anagram_map);

                if anagrams.is_empty() {
                    println!("No anagrams found for '{}'", word);
                } else {
                    println!("Anagrams of '{}': {}", word, anagrams.join(", "));
                }
            }
            "2" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }
    }

    Ok(())
}