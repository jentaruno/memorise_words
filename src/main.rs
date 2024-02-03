use std::io::{self, Write};
use random_word::Lang;
use std::thread;
use std::time::Duration;

fn main() {
    print!("Enter number of words: ");
    io::stdout().flush().expect("Failed to flush stdout");
    // Ask user input, how many words they want
    let mut word_count = String::new();
    io::stdin()
        .read_line(&mut word_count)
        .expect("Failed to read line");
    let word_count: u32 = word_count.trim().parse().expect("Please type a number");
    let mut count: u32 = word_count;
    // Print random words as many word_count as user req
    let mut words: Vec<&str> = Vec::new(); 
    while count > 0 {
        let rand = random_word::gen_len(5, Lang::En);
        if rand.is_some() {
            let word = rand.unwrap();
            words.push(word);
        }
        count -= 1;
    }
    for (i, word) in words.iter().enumerate() {
        println!("{i}: {word}");
    }
    // Countdown timer until words disappear
    countdown(word_count * 5);
    // Let user type the words they remembered
    print!("Enter the words you remember, separated by spaces: ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut points = 0;
    let mut args = String::new();
    io::stdin()
        .read_line(&mut args)
        .expect("Failed to read line");
    let guesses: Vec<&str> = args.split_whitespace().collect();
    // Count points
    for (_i, guess) in guesses.iter().enumerate() {
        if let Some(pos) = words.iter().position(|x| *x == *guess) {
            words.remove(pos);
            points += 1;
        }
    }
    let score = 100 * points / word_count;
    println!("{points}/{word_count} words - that's a score of {score}%!");
    print!("Guessed: {args}");
    print!("Missed:");
    for s in words {
        print!(" {s}");
    }
}

fn countdown(countdown_seconds: u32) {
    println!("Starting countdown:");
    for i in (1..=countdown_seconds).rev() {
        println!("{} seconds remaining...", i);
        print!("\x1B[2K"); // Clear line
        print!("\x1B[1A"); // Move the cursor up one line
        print!("\x1B[2K"); // Clear line
        thread::sleep(Duration::from_secs(1));
    }
    println!("Time's up!");
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}