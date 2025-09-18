use rand::seq::SliceRandom;
use rand::thread_rng;

fn shuffle_japanese_string(input: &str) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    let mut rng = thread_rng();
    chars.shuffle(&mut rng);
    chars.into_iter().collect()
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let shuffled = shuffle_japanese_string(&input);
    println!("{}", shuffled);
}
