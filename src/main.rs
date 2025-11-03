use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;

#[derive(Debug, Deserialize)]
struct WordEntry {
    id: u32,
    word: String,
    category: String,
    difficulty: u32,
}

fn load_words_from_csv(path: &str) -> Result<Vec<WordEntry>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut words = Vec::new();
    
    for result in rdr.deserialize() {
        let record: WordEntry = result?;
        words.push(record);
    }
    
    Ok(words)
}

fn shuffle_japanese_string(input: &str) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    let mut rng = thread_rng();
    chars.shuffle(&mut rng);
    chars.into_iter().collect()
}

fn main() {
    // Load words from CSV
    let words = load_words_from_csv("assets/japanese_dictionary.csv")
        .expect("Failed to load dictionary");
    
    // Randomly select a word
    let mut rng = thread_rng();
    let input = words.choose(&mut rng)
        .expect("Dictionary is empty")
        .word
        .as_str();

    // Shuffle and display
    let original = input.trim();
    let shuffled = shuffle_japanese_string(&input.trim());
    println!("Shuffled: {}", shuffled);

    // Recieve user guess
    let mut guess = String::new();
    std::io::stdin().read_line(&mut guess).expect("Failed to read line");
    let guess = guess.trim();

    if guess == original {
        println!("Correct!");
    } else {
        println!("Incorrect! The correct answer was: {}", original);
    }

    if guess == "give up" {
        println!("The correct answer was: {}", original);
    }
}
