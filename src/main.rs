use std::env;
use std::io::{self, Read, Write};
use random_word::Lang;

fn main() {
    print!("Enter number of words: ");
    io::stdout().flush().expect("Failed to flush stdout");
    // Ask user input, how many words they want
    let mut times = String::new();
    io::stdin()
        .read_line(&mut times)
        .expect("Failed to read line");
    let times: u32 = times.trim().parse().expect("Please type a number");
    let mut count: u32 = times;
    // Print random words as many times as user req
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
    // Let user type the words they remembered
    print!("Enter the words you remember, separated by spaces: ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut points = 0;
    let mut args = String::new();
    io::stdin()
        .read_line(&mut args)
        .expect("Failed to read line");
    let args: Vec<&str> = args.split_whitespace().collect();
    // Count points
    for (i, arg) in args.iter().enumerate() {
        if let Some(pos) = words.iter().position(|x| *x == *arg) {
            words.remove(pos);
            points += 1;
        }
    }
    println!("You got {points}/{times} points!")
}