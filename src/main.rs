use std::collections::HashMap;
use std::fs::File;
use std::env;
use csv::ReaderBuilder;

fn build_anagram_solver(dictionary_folder: &str) -> Result<HashMap<String, Vec<String>>, Box<dyn std::error::Error>> {
    // Explicitly specify the type for the HashMap
    let mut anagram_map: HashMap<String, Vec<String>> = HashMap::new();

    // Iterate over files in the dictionary folder
    for entry in std::fs::read_dir(dictionary_folder)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("csv") {
            let file = File::open(path)?;
            let mut rdr = ReaderBuilder::new()
                .has_headers(false)
                .from_reader(file);

            // Read each record from the CSV file
            for result in rdr.records() {
                let record = result?;
                if let Some(word) = record.get(0) {
                    let word = word.trim().to_lowercase();
                    if !word.is_empty() {
                        let sorted_word = sort_word(&word);
                        // Insert the word into the map
                        anagram_map.entry(sorted_word).or_default().push(word);
                    }
                }
            }
        }
    }

    Ok(anagram_map)
}

// Helper function to sort the letters of a word
fn sort_word(word: &str) -> String {
    let mut chars: Vec<char> = word.chars().collect();
    chars.sort_unstable();
    chars.into_iter().collect()
}

// Find anagrams of a word using the pre-built map
fn find_anagrams(word: &str, anagram_map: &HashMap<String, Vec<String>>) -> Vec<String> {
    let sorted_word = sort_word(word);
    anagram_map.get(&sorted_word).cloned().unwrap_or_default()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <dictionary_folder> <word>", args[0]);
        std::process::exit(1);
    }

    let dictionary_folder = &args[1];
    let word = &args[2];

    println!("Building anagram solver from {}...", dictionary_folder);
    let anagram_map = build_anagram_solver(dictionary_folder)?;
    println!("Anagram solver ready!");

    let anagrams = find_anagrams(word, &anagram_map);

    if anagrams.is_empty() {
        println!("No anagrams found for '{}'", word);
    } else {
        println!("Anagrams of '{}': {}", word, anagrams.join(", "));
    }

    Ok(())
}
